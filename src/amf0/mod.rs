use std::collections::HashMap;

#[cfg(feature = "amf0-avmplus")]
use crate::amf3;

pub mod decoder;

// 2.1 Types Overview
mod marker {
    pub const NUMBER: u8 = 0x00;
    pub const BOOLEAN: u8 = 0x01;
    pub const STRING: u8 = 0x02;
    pub const OBJECT: u8 = 0x03;
    pub const MOVIECLIP: u8 = 0x04; // reserved, not supported
    pub const NULL: u8 = 0x05;
    pub const UNDEFINED: u8 = 0x06;
    pub const REFERENCE: u8 = 0x07;
    pub const ECMA_ARRAY: u8 = 0x08;
    pub const OBJECT_END: u8 = 0x09;
    pub const STRICT_ARRAY: u8 = 0x0A;
    pub const DATE: u8 = 0x0B;
    pub const LONG_STRING: u8 = 0x0C;
    pub const UNSUPPORTED: u8 = 0x0D;
    pub const RECORDSET: u8 = 0x0E; // reserved, not supported
    pub const XML_DOCUMENT: u8 = 0x0F;
    pub const TYPED_OBJECT: u8 = 0x10;
    pub const AVMPLUS_OBJECT: u8 = 0x11;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    #[cfg(feature = "amf0-number")]
    Number(crate::Double),
    #[cfg(feature = "amf0-boolean")]
    Boolean(crate::Boolean),
    #[cfg(feature = "amf0-string")]
    String(crate::String),
    #[cfg(feature = "amf0-object")]
    Object(crate::Object<crate::amf0::Value>),
    Null,
    Undefined,
    #[cfg(feature = "amf0-reference")]
    Reference(u16),
    #[cfg(feature = "amf0-ecma_array")]
    ECMAArray(crate::Property<crate::amf0::Value>),
    ObjectEnd,
    #[cfg(feature = "amf0-strict_array")]
    StrictArray(crate::Array<crate::amf0::Value>),
    #[cfg(feature = "amf0-date")]
    Date(crate::Date), // time-zone is reserved, not supported, should be set to 0x0000
    Unsupported,
    #[cfg(feature = "amf0-xml_document")]
    XMLDocument(crate::String),
    #[cfg(feature = "amf0-avmplus")]
    AVMPlus(crate::amf3::Value),
}
