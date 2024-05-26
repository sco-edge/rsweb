// This file is generated by rust-protobuf 2.28.0. Do not edit
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
//! Generated file from `mahimahi.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct HTTPMessage {
    // message fields
    first_line: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    pub header: ::protobuf::RepeatedField<HTTPHeader>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HTTPMessage {
    fn default() -> &'a HTTPMessage {
        <HTTPMessage as ::protobuf::Message>::default_instance()
    }
}

impl HTTPMessage {
    pub fn new() -> HTTPMessage {
        ::std::default::Default::default()
    }

    // optional bytes first_line = 1;


    pub fn get_first_line(&self) -> &[u8] {
        match self.first_line.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_first_line(&mut self) {
        self.first_line.clear();
    }

    pub fn has_first_line(&self) -> bool {
        self.first_line.is_some()
    }

    // Param is passed by value, moved
    pub fn set_first_line(&mut self, v: ::std::vec::Vec<u8>) {
        self.first_line = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_first_line(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.first_line.is_none() {
            self.first_line.set_default();
        }
        self.first_line.as_mut().unwrap()
    }

    // Take field
    pub fn take_first_line(&mut self) -> ::std::vec::Vec<u8> {
        self.first_line.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // repeated .MahimahiProtobufs.HTTPHeader header = 2;


    pub fn get_header(&self) -> &[HTTPHeader] {
        &self.header
    }
    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ::protobuf::RepeatedField<HTTPHeader>) {
        self.header = v;
    }

    // Mutable pointer to the field.
    pub fn mut_header(&mut self) -> &mut ::protobuf::RepeatedField<HTTPHeader> {
        &mut self.header
    }

    // Take field
    pub fn take_header(&mut self) -> ::protobuf::RepeatedField<HTTPHeader> {
        ::std::mem::replace(&mut self.header, ::protobuf::RepeatedField::new())
    }

    // optional bytes body = 3;


    pub fn get_body(&self) -> &[u8] {
        match self.body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.body.is_none() {
            self.body.set_default();
        }
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::vec::Vec<u8> {
        self.body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for HTTPMessage {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.first_line)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.header)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
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
        if let Some(ref v) = self.first_line.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        for value in &self.header {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.first_line.as_ref() {
            os.write_bytes(1, &v)?;
        }
        for v in &self.header {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.body.as_ref() {
            os.write_bytes(3, &v)?;
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

    fn new() -> HTTPMessage {
        HTTPMessage::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "first_line",
                |m: &HTTPMessage| { &m.first_line },
                |m: &mut HTTPMessage| { &mut m.first_line },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HTTPHeader>>(
                "header",
                |m: &HTTPMessage| { &m.header },
                |m: &mut HTTPMessage| { &mut m.header },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "body",
                |m: &HTTPMessage| { &m.body },
                |m: &mut HTTPMessage| { &mut m.body },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<HTTPMessage>(
                "HTTPMessage",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static HTTPMessage {
        static instance: ::protobuf::rt::LazyV2<HTTPMessage> = ::protobuf::rt::LazyV2::INIT;
        instance.get(HTTPMessage::new)
    }
}

impl ::protobuf::Clear for HTTPMessage {
    fn clear(&mut self) {
        self.first_line.clear();
        self.header.clear();
        self.body.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HTTPMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HTTPMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HTTPHeader {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HTTPHeader {
    fn default() -> &'a HTTPHeader {
        <HTTPHeader as ::protobuf::Message>::default_instance()
    }
}

impl HTTPHeader {
    pub fn new() -> HTTPHeader {
        ::std::default::Default::default()
    }

    // optional bytes key = 1;


    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bytes value = 2;


    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for HTTPHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(ref v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
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

    fn new() -> HTTPHeader {
        HTTPHeader::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "key",
                |m: &HTTPHeader| { &m.key },
                |m: &mut HTTPHeader| { &mut m.key },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "value",
                |m: &HTTPHeader| { &m.value },
                |m: &mut HTTPHeader| { &mut m.value },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<HTTPHeader>(
                "HTTPHeader",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static HTTPHeader {
        static instance: ::protobuf::rt::LazyV2<HTTPHeader> = ::protobuf::rt::LazyV2::INIT;
        instance.get(HTTPHeader::new)
    }
}

impl ::protobuf::Clear for HTTPHeader {
    fn clear(&mut self) {
        self.key.clear();
        self.value.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HTTPHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HTTPHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestResponse {
    // message fields
    ip: ::protobuf::SingularField<::std::string::String>,
    port: ::std::option::Option<u32>,
    scheme: ::std::option::Option<RequestResponse_Scheme>,
    pub request: ::protobuf::SingularPtrField<HTTPMessage>,
    pub response: ::protobuf::SingularPtrField<HTTPMessage>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RequestResponse {
    fn default() -> &'a RequestResponse {
        <RequestResponse as ::protobuf::Message>::default_instance()
    }
}

impl RequestResponse {
    pub fn new() -> RequestResponse {
        ::std::default::Default::default()
    }

    // optional string ip = 1;


    pub fn get_ip(&self) -> &str {
        match self.ip.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::std::string::String {
        if self.ip.is_none() {
            self.ip.set_default();
        }
        self.ip.as_mut().unwrap()
    }

    // Take field
    pub fn take_ip(&mut self) -> ::std::string::String {
        self.ip.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional uint32 port = 2;


    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(0)
    }
    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = ::std::option::Option::Some(v);
    }

    // optional .MahimahiProtobufs.RequestResponse.Scheme scheme = 3;


    pub fn get_scheme(&self) -> RequestResponse_Scheme {
        self.scheme.unwrap_or(RequestResponse_Scheme::HTTP)
    }
    pub fn clear_scheme(&mut self) {
        self.scheme = ::std::option::Option::None;
    }

    pub fn has_scheme(&self) -> bool {
        self.scheme.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scheme(&mut self, v: RequestResponse_Scheme) {
        self.scheme = ::std::option::Option::Some(v);
    }

    // optional .MahimahiProtobufs.HTTPMessage request = 4;


    pub fn get_request(&self) -> &HTTPMessage {
        self.request.as_ref().unwrap_or_else(|| <HTTPMessage as ::protobuf::Message>::default_instance())
    }
    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: HTTPMessage) {
        self.request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request(&mut self) -> &mut HTTPMessage {
        if self.request.is_none() {
            self.request.set_default();
        }
        self.request.as_mut().unwrap()
    }

    // Take field
    pub fn take_request(&mut self) -> HTTPMessage {
        self.request.take().unwrap_or_else(|| HTTPMessage::new())
    }

    // optional .MahimahiProtobufs.HTTPMessage response = 5;


    pub fn get_response(&self) -> &HTTPMessage {
        self.response.as_ref().unwrap_or_else(|| <HTTPMessage as ::protobuf::Message>::default_instance())
    }
    pub fn clear_response(&mut self) {
        self.response.clear();
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: HTTPMessage) {
        self.response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response(&mut self) -> &mut HTTPMessage {
        if self.response.is_none() {
            self.response.set_default();
        }
        self.response.as_mut().unwrap()
    }

    // Take field
    pub fn take_response(&mut self) -> HTTPMessage {
        self.response.take().unwrap_or_else(|| HTTPMessage::new())
    }
}

impl ::protobuf::Message for RequestResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.request {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ip)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.port = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.scheme, 3, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.request)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response)?;
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
        if let Some(ref v) = self.ip.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.port {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.scheme {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(ref v) = self.request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ip.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.port {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.scheme {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&v))?;
        }
        if let Some(ref v) = self.request.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.response.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> RequestResponse {
        RequestResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ip",
                |m: &RequestResponse| { &m.ip },
                |m: &mut RequestResponse| { &mut m.ip },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "port",
                |m: &RequestResponse| { &m.port },
                |m: &mut RequestResponse| { &mut m.port },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RequestResponse_Scheme>>(
                "scheme",
                |m: &RequestResponse| { &m.scheme },
                |m: &mut RequestResponse| { &mut m.scheme },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HTTPMessage>>(
                "request",
                |m: &RequestResponse| { &m.request },
                |m: &mut RequestResponse| { &mut m.request },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HTTPMessage>>(
                "response",
                |m: &RequestResponse| { &m.response },
                |m: &mut RequestResponse| { &mut m.response },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RequestResponse>(
                "RequestResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RequestResponse {
        static instance: ::protobuf::rt::LazyV2<RequestResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RequestResponse::new)
    }
}

impl ::protobuf::Clear for RequestResponse {
    fn clear(&mut self) {
        self.ip.clear();
        self.port = ::std::option::Option::None;
        self.scheme = ::std::option::Option::None;
        self.request.clear();
        self.response.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RequestResponse_Scheme {
    HTTP = 1,
    HTTPS = 2,
}

impl ::protobuf::ProtobufEnum for RequestResponse_Scheme {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RequestResponse_Scheme> {
        match value {
            1 => ::std::option::Option::Some(RequestResponse_Scheme::HTTP),
            2 => ::std::option::Option::Some(RequestResponse_Scheme::HTTPS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RequestResponse_Scheme] = &[
            RequestResponse_Scheme::HTTP,
            RequestResponse_Scheme::HTTPS,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<RequestResponse_Scheme>("RequestResponse.Scheme", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for RequestResponse_Scheme {
}

// Note, `Default` is implemented although default value is not 0
impl ::std::default::Default for RequestResponse_Scheme {
    fn default() -> Self {
        RequestResponse_Scheme::HTTP
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestResponse_Scheme {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0emahimahi.proto\x12\x11MahimahiProtobufs\"w\n\x0bHTTPMessage\x12\
    \x1d\n\nfirst_line\x18\x01\x20\x01(\x0cR\tfirstLine\x125\n\x06header\x18\
    \x02\x20\x03(\x0b2\x1d.MahimahiProtobufs.HTTPHeaderR\x06header\x12\x12\n\
    \x04body\x18\x03\x20\x01(\x0cR\x04body\"4\n\nHTTPHeader\x12\x10\n\x03key\
    \x18\x01\x20\x01(\x0cR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\
    \x05value\"\x8d\x02\n\x0fRequestResponse\x12\x0e\n\x02ip\x18\x01\x20\x01\
    (\tR\x02ip\x12\x12\n\x04port\x18\x02\x20\x01(\rR\x04port\x12A\n\x06schem\
    e\x18\x03\x20\x01(\x0e2).MahimahiProtobufs.RequestResponse.SchemeR\x06sc\
    heme\x128\n\x07request\x18\x04\x20\x01(\x0b2\x1e.MahimahiProtobufs.HTTPM\
    essageR\x07request\x12:\n\x08response\x18\x05\x20\x01(\x0b2\x1e.Mahimahi\
    Protobufs.HTTPMessageR\x08response\"\x1d\n\x06Scheme\x12\x08\n\x04HTTP\
    \x10\x01\x12\t\n\x05HTTPS\x10\x02\
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