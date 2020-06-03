## xuper-rust-sdk

A Xuperchain SDK by rust, especially for TEE(Intel SGX/ARM TZ) application.

## Requirements

XuperChain 3.7

## Function

- [x] load account
- [x] Transfer
- [x] Contract Invoke/Query
- [] balance

## Notices when serializing

In protos/xchain.rs:
* Serialize enum as number: https://serde.rs/enum-number.html
* #[serde(default)]
* crate::wallet::* 


## Test
```
cargo test -- --test-threads 1
```
