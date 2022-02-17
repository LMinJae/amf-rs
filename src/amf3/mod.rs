pub mod decoder;

// 3.1 Overview
mod marker {
    pub const UNDEFINED     : u8 = 0x00;
    pub const NULL          : u8 = 0x01;
    pub const FALSE         : u8 = 0x02;
    pub const TRUE          : u8 = 0x03;
    pub const INTEGER       : u8 = 0x04;
    pub const DOUBLE        : u8 = 0x05;
    pub const STRING        : u8 = 0x06;
    pub const XML_DOC       : u8 = 0x07;
    pub const DATE          : u8 = 0x08;
    pub const ARRAY         : u8 = 0x09;
    pub const OBJECT        : u8 = 0x0A;
    pub const XML           : u8 = 0x0B;
    pub const BYTE_ARRAY    : u8 = 0x0C;
    pub const VECTOR_INT    : u8 = 0x0D;
    pub const VECTOR_UINT   : u8 = 0x0E;
    pub const VECTOR_DOUBLE : u8 = 0x0F;
    pub const VECTOR_OBJECT : u8 = 0x10;
    pub const DICTIONARY    : u8 = 0x11;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Undefined,
    Null,
    False,
    True,
    #[cfg(feature = "amf3-integer")]
    Integer(i32),
    #[cfg(feature = "amf3-double")]
    Double(f64),
}
