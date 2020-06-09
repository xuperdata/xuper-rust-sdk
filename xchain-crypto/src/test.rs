// Copyright 2015-2016 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

//! Testing framework.
//!
//! Unlike the rest of *ring*, this testing framework uses panics pretty
//! liberally. It was originally designed for internal use--it drives most of
//! *ring*'s internal tests, and so it is optimized for getting *ring*'s tests
//! written quickly at the expense of some usability. The documentation is
//! lacking. The best way to learn it is to look at some examples. The digest
//! tests are the most complicated because they use named sections. Other tests
//! avoid named sections and so are easier to understand.
//!
//! # Examples
//!
//! ## Writing Tests
//!
//! Input files look like this:
//!
//! ```text
//! # This is a comment.
//!
//! HMAC = SHA1
//! Input = "My test data"
//! Key = ""
//! Output = 61afdecb95429ef494d61fdee15990cabf0826fc
//!
//! HMAC = SHA256
//! Input = "Sample message for keylen<blocklen"
//! Key = 000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F
//! Output = A28CF43130EE696A98F14A37678B56BCFCBDD9E5CF69717FECF5480F0EBDF790
//! ```
//!
//! Test cases are separated with blank lines. Note how the bytes of the `Key`
//! attribute are specified as a quoted string in the first test case and as
//! hex in the second test case; you can use whichever form is more convenient
//! and you can mix and match within the same file. The empty sequence of bytes
//! can only be represented with the quoted string form (`""`).
//!
//! Here's how you would consume the test data:
//!
//! ```ignore
//! use ring::test;
//!
//! test::run(test::test_file!("hmac_tests.txt"), |section, test_case| {
//!     assert_eq!(section, ""); // This test doesn't use named sections.
//!
//!     let digest_alg = test_case.consume_digest_alg("HMAC");
//!     let input = test_case.consume_bytes("Input");
//!     let key = test_case.consume_bytes("Key");
//!     let output = test_case.consume_bytes("Output");
//!
//!     // Do the actual testing here
//! });
//! ```
//!
//! Note that `consume_digest_alg` automatically maps the string "SHA1" to a
//! reference to `digest::SHA1_FOR_LEGACY_USE_ONLY`, "SHA256" to
//! `digest::SHA256`, etc.
//!
//! ## Output When a Test Fails
//!
//! When a test case fails, the framework automatically prints out the test
//! case. If the test case failed with a panic, then the backtrace of the panic
//! will be printed too. For example, let's say the failing test case looks
//! like this:
//!
//! ```text
//! Curve = P-256
//! a = 2b11cb945c8cf152ffa4c9c2b1c965b019b35d0b7626919ef0ae6cb9d232f8af
//! b = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
//! r = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
//! ```
//! If the test fails, this will be printed (if `$RUST_BACKTRACE` is `1`):
//!
//! ```text
//! src/example_tests.txt: Test panicked.
//! Curve = P-256
//! a = 2b11cb945c8cf152ffa4c9c2b1c965b019b35d0b7626919ef0ae6cb9d232f8af
//! b = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
//! r = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c
//! thread 'example_test' panicked at 'Test failed.', src\test.rs:206
//! stack backtrace:
//!    0:     0x7ff654a05c7c - std::rt::lang_start::h61f4934e780b4dfc
//!    1:     0x7ff654a04f32 - std::rt::lang_start::h61f4934e780b4dfc
//!    2:     0x7ff6549f505d - std::panicking::rust_panic_with_hook::hfe203e3083c2b544
//!    3:     0x7ff654a0825b - rust_begin_unwind
//!    4:     0x7ff6549f63af - std::panicking::begin_panic_fmt::h484cd47786497f03
//!    5:     0x7ff654a07e9b - rust_begin_unwind
//!    6:     0x7ff654a0ae95 - core::panicking::panic_fmt::h257ceb0aa351d801
//!    7:     0x7ff654a0b190 - core::panicking::panic::h4bb1497076d04ab9
//!    8:     0x7ff65496dc41 - from_file<closure>
//!                         at C:\Users\Example\example\<core macros>:4
//!    9:     0x7ff65496d49c - example_test
//!                         at C:\Users\Example\example\src\example.rs:652
//!   10:     0x7ff6549d192a - test::stats::Summary::new::ha139494ed2e4e01f
//!   11:     0x7ff6549d51a2 - test::stats::Summary::new::ha139494ed2e4e01f
//!   12:     0x7ff654a0a911 - _rust_maybe_catch_panic
//!   13:     0x7ff6549d56dd - test::stats::Summary::new::ha139494ed2e4e01f
//!   14:     0x7ff654a03783 - std::sys::thread::Thread::new::h2b08da6cd2517f79
//!   15:     0x7ff968518101 - BaseThreadInitThunk
//! ```
//!
//! Notice that the output shows the name of the data file
//! (`src/example_tests.txt`), the test inputs that led to the failure, and the
//! stack trace to the line in the test code that panicked: entry 9 in the
//! stack trace pointing to line 652 of the file `example.rs`.

#[cfg(feature = "alloc")]
use std::{format, string::String, vec::Vec};

#[cfg(feature = "alloc")]
use crate::{bits, errors::Result};

#[cfg(feature = "alloc")]
use ring::digest;

#[cfg(any(feature = "std", feature = "test_logging"))]
extern crate std;

/// `compile_time_assert_clone::<T>();` fails to compile if `T` doesn't
/// implement `Clone`.
pub fn compile_time_assert_clone<T: Clone>() {}

/// `compile_time_assert_copy::<T>();` fails to compile if `T` doesn't
/// implement `Copy`.
pub fn compile_time_assert_copy<T: Copy>() {}

/// `compile_time_assert_send::<T>();` fails to compile if `T` doesn't
/// implement `Send`.
pub fn compile_time_assert_send<T: Send>() {}

/// `compile_time_assert_sync::<T>();` fails to compile if `T` doesn't
/// implement `Sync`.
pub fn compile_time_assert_sync<T: Sync>() {}

/// `compile_time_assert_std_error_error::<T>();` fails to compile if `T`
/// doesn't implement `std::error::Error`.
#[cfg(feature = "std")]
pub fn compile_time_assert_std_error_error<T: std::error::Error>() {}

/// A test case. A test case consists of a set of named attributes. Every
/// attribute in the test case must be consumed exactly once; this helps catch
/// typos and omissions.
///
/// Requires the `alloc` default feature to be enabled.
#[cfg(feature = "alloc")]
#[derive(Debug)]
pub struct TestCase {
    attributes: Vec<(String, String, bool)>,
}

#[cfg(feature = "alloc")]
impl TestCase {
    /// Maps the string "true" to true and the string "false" to false.
    pub fn consume_bool(&mut self, key: &str) -> bool {
        match self.consume_string(key).as_ref() {
            "true" => true,
            "false" => false,
            s => panic!("Invalid bool value: {}", s),
        }
    }

    /// Maps the strings "SHA1", "SHA256", "SHA384", and "SHA512" to digest
    /// algorithms, maps "SHA224" to `None`, and panics on other (erroneous)
    /// inputs. "SHA224" is mapped to None because *ring* intentionally does
    /// not support SHA224, but we need to consume test vectors from NIST that
    /// have SHA224 vectors in them.
    pub fn consume_digest_alg(&mut self, key: &str) -> Option<&'static digest::Algorithm> {
        let name = self.consume_string(key);
        match name.as_ref() {
            "SHA1" => Some(&digest::SHA1_FOR_LEGACY_USE_ONLY),
            "SHA224" => None, // We actively skip SHA-224 support.
            "SHA256" => Some(&digest::SHA256),
            "SHA384" => Some(&digest::SHA384),
            "SHA512" => Some(&digest::SHA512),
            "SHA512_256" => Some(&digest::SHA512_256),
            _ => panic!("Unsupported digest algorithm: {}", name),
        }
    }

    /// Returns the value of an attribute that is encoded as a sequence of an
    /// even number of hex digits, or as a double-quoted UTF-8 string. The
    /// empty (zero-length) value is represented as "".
    pub fn consume_bytes(&mut self, key: &str) -> Vec<u8> {
        let s = self.consume_string(key);
        if s.starts_with('\"') {
            // The value is a quoted UTF-8 string.

            let mut bytes = Vec::with_capacity(s.as_bytes().len() - 2);
            let mut s = s.as_bytes().iter().skip(1);
            loop {
                let b = match s.next() {
                    Some(b'\\') => {
                        match s.next() {
                            // We don't allow all octal escape sequences, only "\0" for null.
                            Some(b'0') => 0u8,
                            Some(b't') => b'\t',
                            Some(b'n') => b'\n',
                            // "\xHH"
                            Some(b'x') => {
                                let hi = s.next().expect("Invalid hex escape sequence in string.");
                                let lo = s.next().expect("Invalid hex escape sequence in string.");
                                if let (Ok(hi), Ok(lo)) = (from_hex_digit(*hi), from_hex_digit(*lo))
                                {
                                    (hi << 4) | lo
                                } else {
                                    panic!("Invalid hex escape sequence in string.");
                                }
                            }
                            _ => {
                                panic!("Invalid hex escape sequence in string.");
                            }
                        }
                    }
                    Some(b'"') => {
                        if s.next().is_some() {
                            panic!("characters after the closing quote of a quoted string.");
                        }
                        break;
                    }
                    Some(b) => *b,
                    None => panic!("Missing terminating '\"' in string literal."),
                };
                bytes.push(b);
            }
            bytes
        } else {
            // The value is hex encoded.
            match from_hex(&s) {
                Ok(s) => s,
                Err(err_str) => {
                    panic!("{} in {}", err_str, s);
                }
            }
        }
    }

    /// Returns the value of an attribute that is an integer, in decimal
    /// notation.
    pub fn consume_usize(&mut self, key: &str) -> usize {
        let s = self.consume_string(key);
        s.parse::<usize>().unwrap()
    }

    /// Returns the value of an attribute that is an integer, in decimal
    /// notation, as a bit length.
    #[cfg(feature = "alloc")]
    pub fn consume_usize_bits(&mut self, key: &str) -> bits::BitLength {
        let s = self.consume_string(key);
        let bits = s.parse::<usize>().unwrap();
        bits::BitLength::from_usize_bits(bits)
    }

    /// Returns the raw value of an attribute, without any unquoting or
    /// other interpretation.
    pub fn consume_string(&mut self, key: &str) -> String {
        self.consume_optional_string(key)
            .unwrap_or_else(|| panic!("No attribute named \"{}\"", key))
    }

    /// Like `consume_string()` except it returns `None` if the test case
    /// doesn't have the attribute.
    pub fn consume_optional_string(&mut self, key: &str) -> Option<String> {
        for (name, value, consumed) in &mut self.attributes {
            if key == name {
                if *consumed {
                    panic!("Attribute {} was already consumed", key);
                }
                *consumed = true;
                return Some(value.clone());
            }
        }
        None
    }
}

/// References a test input file.
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! test_file {
    ($file_name:expr) => {
        crate::test::File {
            file_name: $file_name,
            contents: include_str!($file_name),
        }
    };
}

/// A test input file.
#[cfg(feature = "alloc")]
pub struct File<'a> {
    /// The name (path) of the file.
    pub file_name: &'a str,

    /// The contents of the file.
    pub contents: &'a str,
}

/// Parses test cases out of the given file, calling `f` on each vector until
/// `f` fails or until all the test vectors have been read. `f` can indicate
/// failure either by returning `Err()` or by panicking.
///
/// Requires the `alloc` default feature to be enabled
#[cfg(feature = "alloc")]
pub fn run<F>(test_file: File, mut f: F)
where
    F: FnMut(&str, &mut TestCase) -> Result<()>,
{
    let lines = &mut test_file.contents.lines();

    let mut current_section = String::from("");
    let mut failed = false;

    while let Some(mut test_case) = parse_test_case(&mut current_section, lines) {
        let result = match f(&current_section, &mut test_case) {
            Ok(()) => {
                if !test_case
                    .attributes
                    .iter()
                    .any(|&(_, _, consumed)| !consumed)
                {
                    Ok(())
                } else {
                    failed = true;
                    Err("Test didn't consume all attributes.")
                }
            }
            Err(_) => Err("Test returned Err(error::Unspecified)."),
        };

        if result.is_err() {
            failed = true;
        }

        #[cfg(feature = "test_logging")]
        {
            if let Err(msg) = result {
                std::println!("{}: {}", test_file.file_name, msg);

                for (name, value, consumed) in test_case.attributes {
                    let consumed_str = if consumed { "" } else { " (unconsumed)" };
                    std::println!("{}{} = {}", name, consumed_str, value);
                }
            };
        }
    }

    if failed {
        panic!("Test failed.")
    }
}

/// Decode an string of hex digits into a sequence of bytes. The input must
/// have an even number of digits.
#[cfg(feature = "alloc")]
pub fn from_hex(hex_str: &str) -> std::result::Result<Vec<u8>, String> {
    if hex_str.len() % 2 != 0 {
        return Err(String::from(
            "Hex string does not have an even number of digits",
        ));
    }

    let mut result = Vec::with_capacity(hex_str.len() / 2);
    for digits in hex_str.as_bytes().chunks(2) {
        let hi = from_hex_digit(digits[0])?;
        let lo = from_hex_digit(digits[1])?;
        result.push((hi * 0x10) | lo);
    }
    Ok(result)
}

#[cfg(feature = "alloc")]
fn from_hex_digit(d: u8) -> std::result::Result<u8, String> {
    if d >= b'0' && d <= b'9' {
        Ok(d - b'0')
    } else if d >= b'a' && d <= b'f' {
        Ok(d - b'a' + 10u8)
    } else if d >= b'A' && d <= b'F' {
        Ok(d - b'A' + 10u8)
    } else {
        Err(format!("Invalid hex digit '{}'", d as char))
    }
}

#[cfg(feature = "alloc")]
fn parse_test_case(
    current_section: &mut String,
    lines: &mut dyn Iterator<Item = &str>,
) -> Option<TestCase> {
    let mut attributes = Vec::new();

    let mut is_first_line = true;
    loop {
        let line = lines.next();

        #[cfg(feature = "test_logging")]
        {
            if let Some(text) = &line {
                std::println!("Line: {}", text);
            }
        }

        match line {
            // If we get to EOF when we're not in the middle of a test case,
            // then we're done.
            None if is_first_line => {
                return None;
            }

            // End of the file on a non-empty test cases ends the test case.
            None => {
                return Some(TestCase { attributes });
            }

            // A blank line ends a test case if the test case isn't empty.
            Some(ref line) if line.is_empty() => {
                if !is_first_line {
                    return Some(TestCase { attributes });
                }
                // Ignore leading blank lines.
            }

            // Comments start with '#'; ignore them.
            Some(ref line) if line.starts_with('#') => (),

            Some(ref line) if line.starts_with('[') => {
                assert!(is_first_line);
                assert!(line.ends_with(']'));
                current_section.truncate(0);
                current_section.push_str(line);
                let _ = current_section.pop();
                let _ = current_section.remove(0);
            }

            Some(ref line) => {
                is_first_line = false;

                let parts: Vec<&str> = line.splitn(2, " = ").collect();
                if parts.len() != 2 {
                    panic!("Syntax error: Expected Key = Value.");
                };

                let key = parts[0].trim();
                let value = parts[1].trim();

                // Don't allow the value to be ommitted. An empty value can be
                // represented as an empty quoted string.
                assert_ne!(value.len(), 0);

                // Checking is_none() ensures we don't accept duplicate keys.
                attributes.push((String::from(key), String::from(value), false));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{errors::*, test};

    #[test]
    fn one_ok() {
        test::run(test_file!("test_1_tests.txt"), |_, test_case| {
            let _ = test_case.consume_string("Key");
            Ok(())
        });
    }

    #[test]
    #[should_panic(expected = "Test failed.")]
    fn one_err() {
        test::run(test_file!("test_1_tests.txt"), |_, test_case| {
            let _ = test_case.consume_string("Key");
            Err(Error::from(ErrorKind::CryptoError))
        });
    }

    #[test]
    #[should_panic(expected = "Oh noes!")]
    fn one_panics() {
        test::run(test_file!("test_1_tests.txt"), |_, test_case| {
            let _ = test_case.consume_string("Key");
            panic!("Oh noes!");
        });
    }

    #[test]
    #[should_panic(expected = "Test failed.")]
    fn first_err() {
        err_one(0)
    }

    #[test]
    #[should_panic(expected = "Test failed.")]
    fn middle_err() {
        err_one(1)
    }

    #[test]
    #[should_panic(expected = "Test failed.")]
    fn last_err() {
        err_one(2)
    }

    fn err_one(test_to_fail: usize) {
        let mut n = 0;
        test::run(test_file!("test_3_tests.txt"), |_, test_case| {
            let _ = test_case.consume_string("Key");
            let result = if n != test_to_fail {
                Ok(())
            } else {
                Err(Error::from(ErrorKind::CryptoError))
            };
            n += 1;
            result
        });
    }

    #[test]
    #[should_panic(expected = "Oh Noes!")]
    fn first_panic() {
        panic_one(0)
    }

    #[test]
    #[should_panic(expected = "Oh Noes!")]
    fn middle_panic() {
        panic_one(1)
    }

    #[test]
    #[should_panic(expected = "Oh Noes!")]
    fn last_panic() {
        panic_one(2)
    }

    fn panic_one(test_to_fail: usize) {
        let mut n = 0;
        test::run(test_file!("test_3_tests.txt"), |_, test_case| {
            let _ = test_case.consume_string("Key");
            if n == test_to_fail {
                panic!("Oh Noes!");
            };
            n += 1;
            Ok(())
        });
    }

    #[test]
    #[should_panic(expected = "Syntax error: Expected Key = Value.")]
    fn syntax_error() {
        test::run(test_file!("test_1_syntax_error_tests.txt"), |_, _| Ok(()));
    }
}