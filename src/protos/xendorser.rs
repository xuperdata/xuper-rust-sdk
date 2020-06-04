// This file is generated by rust-protobuf 2.14.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `xendorser.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_14_0;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct EndorserRequest {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::xchain::Header>,
    pub RequestName: ::std::string::String,
    pub BcName: ::std::string::String,
    pub Fee: ::protobuf::SingularPtrField<super::xchain::Transaction>,
    #[serde(serialize_with = "crate::encoder::serialize_bytes")]
    #[serde(deserialize_with = "crate::encoder::deserialize_bytes")]
    pub RequestData: ::std::vec::Vec<u8>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EndorserRequest {
    fn default() -> &'a EndorserRequest {
        <EndorserRequest as ::protobuf::Message>::default_instance()
    }
}

impl EndorserRequest {
    pub fn new() -> EndorserRequest {
        ::std::default::Default::default()
    }

    // .pb.Header header = 1;


    pub fn get_header(&self) -> &super::xchain::Header {
        self.header.as_ref().unwrap_or_else(|| super::xchain::Header::default_instance())
    }
    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::xchain::Header) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::xchain::Header {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::xchain::Header {
        self.header.take().unwrap_or_else(|| super::xchain::Header::new())
    }

    // string RequestName = 2;


    pub fn get_RequestName(&self) -> &str {
        &self.RequestName
    }
    pub fn clear_RequestName(&mut self) {
        self.RequestName.clear();
    }

    // Param is passed by value, moved
    pub fn set_RequestName(&mut self, v: ::std::string::String) {
        self.RequestName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_RequestName(&mut self) -> &mut ::std::string::String {
        &mut self.RequestName
    }

    // Take field
    pub fn take_RequestName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.RequestName, ::std::string::String::new())
    }

    // string BcName = 3;


    pub fn get_BcName(&self) -> &str {
        &self.BcName
    }
    pub fn clear_BcName(&mut self) {
        self.BcName.clear();
    }

    // Param is passed by value, moved
    pub fn set_BcName(&mut self, v: ::std::string::String) {
        self.BcName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_BcName(&mut self) -> &mut ::std::string::String {
        &mut self.BcName
    }

    // Take field
    pub fn take_BcName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.BcName, ::std::string::String::new())
    }

    // .pb.Transaction Fee = 4;


    pub fn get_Fee(&self) -> &super::xchain::Transaction {
        self.Fee.as_ref().unwrap_or_else(|| super::xchain::Transaction::default_instance())
    }
    pub fn clear_Fee(&mut self) {
        self.Fee.clear();
    }

    pub fn has_Fee(&self) -> bool {
        self.Fee.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Fee(&mut self, v: super::xchain::Transaction) {
        self.Fee = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Fee(&mut self) -> &mut super::xchain::Transaction {
        if self.Fee.is_none() {
            self.Fee.set_default();
        }
        self.Fee.as_mut().unwrap()
    }

    // Take field
    pub fn take_Fee(&mut self) -> super::xchain::Transaction {
        self.Fee.take().unwrap_or_else(|| super::xchain::Transaction::new())
    }

    // bytes RequestData = 5;


    pub fn get_RequestData(&self) -> &[u8] {
        &self.RequestData
    }
    pub fn clear_RequestData(&mut self) {
        self.RequestData.clear();
    }

    // Param is passed by value, moved
    pub fn set_RequestData(&mut self, v: ::std::vec::Vec<u8>) {
        self.RequestData = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_RequestData(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.RequestData
    }

    // Take field
    pub fn take_RequestData(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.RequestData, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for EndorserRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.Fee {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.RequestName)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.BcName)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.Fee)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.RequestData)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.RequestName.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.RequestName);
        }
        if !self.BcName.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.BcName);
        }
        if let Some(ref v) = self.Fee.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.RequestData.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.RequestData);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.RequestName.is_empty() {
            os.write_string(2, &self.RequestName)?;
        }
        if !self.BcName.is_empty() {
            os.write_string(3, &self.BcName)?;
        }
        if let Some(ref v) = self.Fee.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.RequestData.is_empty() {
            os.write_bytes(5, &self.RequestData)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EndorserRequest {
        EndorserRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::xchain::Header>>(
                    "header",
                    |m: &EndorserRequest| { &m.header },
                    |m: &mut EndorserRequest| { &mut m.header },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "RequestName",
                    |m: &EndorserRequest| { &m.RequestName },
                    |m: &mut EndorserRequest| { &mut m.RequestName },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "BcName",
                    |m: &EndorserRequest| { &m.BcName },
                    |m: &mut EndorserRequest| { &mut m.BcName },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::xchain::Transaction>>(
                    "Fee",
                    |m: &EndorserRequest| { &m.Fee },
                    |m: &mut EndorserRequest| { &mut m.Fee },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "RequestData",
                    |m: &EndorserRequest| { &m.RequestData },
                    |m: &mut EndorserRequest| { &mut m.RequestData },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<EndorserRequest>(
                    "EndorserRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static EndorserRequest {
        static mut instance: ::protobuf::lazy::Lazy<EndorserRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(EndorserRequest::new)
        }
    }
}

impl ::protobuf::Clear for EndorserRequest {
    fn clear(&mut self) {
        self.header.clear();
        self.RequestName.clear();
        self.BcName.clear();
        self.Fee.clear();
        self.RequestData.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EndorserRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EndorserRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct EndorserResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::xchain::Header>,
    pub ResponseName: ::std::string::String,
    pub EndorserAddress: ::std::string::String,
    pub EndorserSign: ::protobuf::SingularPtrField<super::xchain::SignatureInfo>,
    pub ResponseData: ::std::vec::Vec<u8>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EndorserResponse {
    fn default() -> &'a EndorserResponse {
        <EndorserResponse as ::protobuf::Message>::default_instance()
    }
}

impl EndorserResponse {
    pub fn new() -> EndorserResponse {
        ::std::default::Default::default()
    }

    // .pb.Header header = 1;


    pub fn get_header(&self) -> &super::xchain::Header {
        self.header.as_ref().unwrap_or_else(|| super::xchain::Header::default_instance())
    }
    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::xchain::Header) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::xchain::Header {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::xchain::Header {
        self.header.take().unwrap_or_else(|| super::xchain::Header::new())
    }

    // string ResponseName = 2;


    pub fn get_ResponseName(&self) -> &str {
        &self.ResponseName
    }
    pub fn clear_ResponseName(&mut self) {
        self.ResponseName.clear();
    }

    // Param is passed by value, moved
    pub fn set_ResponseName(&mut self, v: ::std::string::String) {
        self.ResponseName = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ResponseName(&mut self) -> &mut ::std::string::String {
        &mut self.ResponseName
    }

    // Take field
    pub fn take_ResponseName(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ResponseName, ::std::string::String::new())
    }

    // string EndorserAddress = 3;


    pub fn get_EndorserAddress(&self) -> &str {
        &self.EndorserAddress
    }
    pub fn clear_EndorserAddress(&mut self) {
        self.EndorserAddress.clear();
    }

    // Param is passed by value, moved
    pub fn set_EndorserAddress(&mut self, v: ::std::string::String) {
        self.EndorserAddress = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_EndorserAddress(&mut self) -> &mut ::std::string::String {
        &mut self.EndorserAddress
    }

    // Take field
    pub fn take_EndorserAddress(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.EndorserAddress, ::std::string::String::new())
    }

    // .pb.SignatureInfo EndorserSign = 4;


    pub fn get_EndorserSign(&self) -> &super::xchain::SignatureInfo {
        self.EndorserSign.as_ref().unwrap_or_else(|| super::xchain::SignatureInfo::default_instance())
    }
    pub fn clear_EndorserSign(&mut self) {
        self.EndorserSign.clear();
    }

    pub fn has_EndorserSign(&self) -> bool {
        self.EndorserSign.is_some()
    }

    // Param is passed by value, moved
    pub fn set_EndorserSign(&mut self, v: super::xchain::SignatureInfo) {
        self.EndorserSign = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_EndorserSign(&mut self) -> &mut super::xchain::SignatureInfo {
        if self.EndorserSign.is_none() {
            self.EndorserSign.set_default();
        }
        self.EndorserSign.as_mut().unwrap()
    }

    // Take field
    pub fn take_EndorserSign(&mut self) -> super::xchain::SignatureInfo {
        self.EndorserSign.take().unwrap_or_else(|| super::xchain::SignatureInfo::new())
    }

    // bytes ResponseData = 5;


    pub fn get_ResponseData(&self) -> &[u8] {
        &self.ResponseData
    }
    pub fn clear_ResponseData(&mut self) {
        self.ResponseData.clear();
    }

    // Param is passed by value, moved
    pub fn set_ResponseData(&mut self, v: ::std::vec::Vec<u8>) {
        self.ResponseData = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ResponseData(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.ResponseData
    }

    // Take field
    pub fn take_ResponseData(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.ResponseData, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for EndorserResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.EndorserSign {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ResponseName)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.EndorserAddress)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.EndorserSign)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.ResponseData)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.ResponseName.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.ResponseName);
        }
        if !self.EndorserAddress.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.EndorserAddress);
        }
        if let Some(ref v) = self.EndorserSign.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.ResponseData.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.ResponseData);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.ResponseName.is_empty() {
            os.write_string(2, &self.ResponseName)?;
        }
        if !self.EndorserAddress.is_empty() {
            os.write_string(3, &self.EndorserAddress)?;
        }
        if let Some(ref v) = self.EndorserSign.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.ResponseData.is_empty() {
            os.write_bytes(5, &self.ResponseData)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EndorserResponse {
        EndorserResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::xchain::Header>>(
                    "header",
                    |m: &EndorserResponse| { &m.header },
                    |m: &mut EndorserResponse| { &mut m.header },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ResponseName",
                    |m: &EndorserResponse| { &m.ResponseName },
                    |m: &mut EndorserResponse| { &mut m.ResponseName },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "EndorserAddress",
                    |m: &EndorserResponse| { &m.EndorserAddress },
                    |m: &mut EndorserResponse| { &mut m.EndorserAddress },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::xchain::SignatureInfo>>(
                    "EndorserSign",
                    |m: &EndorserResponse| { &m.EndorserSign },
                    |m: &mut EndorserResponse| { &mut m.EndorserSign },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ResponseData",
                    |m: &EndorserResponse| { &m.ResponseData },
                    |m: &mut EndorserResponse| { &mut m.ResponseData },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<EndorserResponse>(
                    "EndorserResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static EndorserResponse {
        static mut instance: ::protobuf::lazy::Lazy<EndorserResponse> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(EndorserResponse::new)
        }
    }
}

impl ::protobuf::Clear for EndorserResponse {
    fn clear(&mut self) {
        self.header.clear();
        self.ResponseName.clear();
        self.EndorserAddress.clear();
        self.EndorserSign.clear();
        self.ResponseData.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EndorserResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EndorserResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fxendorser.proto\x12\x02pb\x1a\x0cxchain.proto\"\xb4\x01\n\x0fEndor\
    serRequest\x12\"\n\x06header\x18\x01\x20\x01(\x0b2\n.pb.HeaderR\x06heade\
    r\x12\x20\n\x0bRequestName\x18\x02\x20\x01(\tR\x0bRequestName\x12\x16\n\
    \x06BcName\x18\x03\x20\x01(\tR\x06BcName\x12!\n\x03Fee\x18\x04\x20\x01(\
    \x0b2\x0f.pb.TransactionR\x03Fee\x12\x20\n\x0bRequestData\x18\x05\x20\
    \x01(\x0cR\x0bRequestData\"\xdf\x01\n\x10EndorserResponse\x12\"\n\x06hea\
    der\x18\x01\x20\x01(\x0b2\n.pb.HeaderR\x06header\x12\"\n\x0cResponseName\
    \x18\x02\x20\x01(\tR\x0cResponseName\x12(\n\x0fEndorserAddress\x18\x03\
    \x20\x01(\tR\x0fEndorserAddress\x125\n\x0cEndorserSign\x18\x04\x20\x01(\
    \x0b2\x11.pb.SignatureInfoR\x0cEndorserSign\x12\"\n\x0cResponseData\x18\
    \x05\x20\x01(\x0cR\x0cResponseData2H\n\txendorser\x12;\n\x0cEndorserCall\
    \x12\x13.pb.EndorserRequest\x1a\x14.pb.EndorserResponse\"\0b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
