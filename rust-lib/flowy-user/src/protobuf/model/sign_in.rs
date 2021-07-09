// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `sign_in.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct SignInRequest {
    // message fields
    pub email: ::std::string::String,
    pub password: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SignInRequest {
    fn default() -> &'a SignInRequest {
        <SignInRequest as ::protobuf::Message>::default_instance()
    }
}

impl SignInRequest {
    pub fn new() -> SignInRequest {
        ::std::default::Default::default()
    }

    // string email = 1;


    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.email, ::std::string::String::new())
    }

    // string password = 2;


    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }
}

impl ::protobuf::Message for SignInRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
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
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.email);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.email.is_empty() {
            os.write_string(1, &self.email)?;
        }
        if !self.password.is_empty() {
            os.write_string(2, &self.password)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SignInRequest {
        SignInRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "email",
                |m: &SignInRequest| { &m.email },
                |m: &mut SignInRequest| { &mut m.email },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "password",
                |m: &SignInRequest| { &m.password },
                |m: &mut SignInRequest| { &mut m.password },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SignInRequest>(
                "SignInRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SignInRequest {
        static instance: ::protobuf::rt::LazyV2<SignInRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SignInRequest::new)
    }
}

impl ::protobuf::Clear for SignInRequest {
    fn clear(&mut self) {
        self.email.clear();
        self.password.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignInRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignInRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignInParams {
    // message fields
    pub email: ::std::string::String,
    pub password: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SignInParams {
    fn default() -> &'a SignInParams {
        <SignInParams as ::protobuf::Message>::default_instance()
    }
}

impl SignInParams {
    pub fn new() -> SignInParams {
        ::std::default::Default::default()
    }

    // string email = 1;


    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.email, ::std::string::String::new())
    }

    // string password = 2;


    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }
}

impl ::protobuf::Message for SignInParams {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
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
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.email);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.email.is_empty() {
            os.write_string(1, &self.email)?;
        }
        if !self.password.is_empty() {
            os.write_string(2, &self.password)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SignInParams {
        SignInParams::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "email",
                |m: &SignInParams| { &m.email },
                |m: &mut SignInParams| { &mut m.email },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "password",
                |m: &SignInParams| { &m.password },
                |m: &mut SignInParams| { &mut m.password },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SignInParams>(
                "SignInParams",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SignInParams {
        static instance: ::protobuf::rt::LazyV2<SignInParams> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SignInParams::new)
    }
}

impl ::protobuf::Clear for SignInParams {
    fn clear(&mut self) {
        self.email.clear();
        self.password.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignInParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignInParams {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignInResponse {
    // message fields
    pub is_success: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SignInResponse {
    fn default() -> &'a SignInResponse {
        <SignInResponse as ::protobuf::Message>::default_instance()
    }
}

impl SignInResponse {
    pub fn new() -> SignInResponse {
        ::std::default::Default::default()
    }

    // bool is_success = 1;


    pub fn get_is_success(&self) -> bool {
        self.is_success
    }
    pub fn clear_is_success(&mut self) {
        self.is_success = false;
    }

    // Param is passed by value, moved
    pub fn set_is_success(&mut self, v: bool) {
        self.is_success = v;
    }
}

impl ::protobuf::Message for SignInResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_success = tmp;
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
        if self.is_success != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.is_success != false {
            os.write_bool(1, self.is_success)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SignInResponse {
        SignInResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "is_success",
                |m: &SignInResponse| { &m.is_success },
                |m: &mut SignInResponse| { &mut m.is_success },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SignInResponse>(
                "SignInResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SignInResponse {
        static instance: ::protobuf::rt::LazyV2<SignInResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SignInResponse::new)
    }
}

impl ::protobuf::Clear for SignInResponse {
    fn clear(&mut self) {
        self.is_success = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignInResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignInResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rsign_in.proto\"A\n\rSignInRequest\x12\x14\n\x05email\x18\x01\x20\x01\
    (\tR\x05email\x12\x1a\n\x08password\x18\x02\x20\x01(\tR\x08password\"@\n\
    \x0cSignInParams\x12\x14\n\x05email\x18\x01\x20\x01(\tR\x05email\x12\x1a\
    \n\x08password\x18\x02\x20\x01(\tR\x08password\"/\n\x0eSignInResponse\
    \x12\x1d\n\nis_success\x18\x01\x20\x01(\x08R\tisSuccessJ\xed\x02\n\x06\
    \x12\x04\0\0\x0c\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\
    \x04\x02\0\x05\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x15\n\x0b\n\x04\
    \x04\0\x02\0\x12\x03\x03\x04\x15\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\
    \x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\x10\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\x03\x13\x14\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\
    \x18\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x03\x04\x0b\x13\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\
    \x04\x16\x17\n\n\n\x02\x04\x01\x12\x04\x06\0\t\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03\x06\x08\x14\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x07\x04\x15\n\x0c\
    \n\x05\x04\x01\x02\0\x05\x12\x03\x07\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\
    \x12\x03\x07\x0b\x10\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x07\x13\x14\n\
    \x0b\n\x04\x04\x01\x02\x01\x12\x03\x08\x04\x18\n\x0c\n\x05\x04\x01\x02\
    \x01\x05\x12\x03\x08\x04\n\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x08\
    \x0b\x13\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x08\x16\x17\n\n\n\x02\
    \x04\x02\x12\x04\n\0\x0c\x01\n\n\n\x03\x04\x02\x01\x12\x03\n\x08\x16\n\
    \x0b\n\x04\x04\x02\x02\0\x12\x03\x0b\x04\x18\n\x0c\n\x05\x04\x02\x02\0\
    \x05\x12\x03\x0b\x04\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x0b\t\x13\
    \n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x0b\x16\x17b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}