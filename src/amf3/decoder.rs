use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Error, ErrorKind, Read};

use super::{marker, Value};

pub fn from_bytes<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    let marker = r.read_u8()?;
    match marker {
        marker::UNDEFINED => Ok(Value::Undefined),
        marker::NULL => Ok(Value::Null),
        marker::FALSE => Ok(Value::False),
        marker::TRUE => Ok(Value::True),
        #[cfg(feature = "amf3-integer")]
        marker::INTEGER => parse_integer(r),
        #[cfg(feature = "amf3-double")]
        marker::DOUBLE => parse_double(r),
        #[cfg(feature = "amf3-string")]
        marker::STRING => parse_string(r),
        #[cfg(feature = "amf3-xml_doc")]
        marker::XML_DOC => parse_xml_document(r),
        #[cfg(feature = "amf3-date")]
        marker::DATE => parse_date(r),
        #[cfg(feature = "amf3-array")]
        marker::ARRAY => parse_array(r),
        #[cfg(feature = "amf3-object")]
        marker::OBJECT => parse_object(r),
        #[cfg(feature = "amf3-xml")]
        marker::XML => parse_xml(r),
        #[cfg(feature = "amf3-byte_array")]
        marker::BYTE_ARRAY => parse_byte_array(r),
        #[cfg(feature = "amf3-vector_int")]
        marker::VECTOR_INT => parse_vector_int(r),
        #[cfg(feature = "amf3-vector_uint")]
        marker::VECTOR_UINT => parse_vector_uint(r),
        #[cfg(feature = "amf3-vector_double")]
        marker::VECTOR_DOUBLE => parse_vector_double(r),
        #[cfg(feature = "amf3-vector_object")]
        marker::VECTOR_OBJECT => parse_vector_object(r),
        #[cfg(feature = "amf3-dictionary")]
        marker::DICTIONARY => parse_dictionary(r),
        _ => Err(Error::new(ErrorKind::Other, "Unsupported marker")),
    }
}

#[cfg(feature = "amf3-integer")]
fn parse_integer<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    let u29 = parse_u29(r)? as i32;
    let i = if 0 != (1 << 28) & u29 {
        u29 - (1 << 29)
    } else {
        u29
    };

    Ok(Value::Integer(i))
}

#[cfg(feature = "amf3-double")]
fn parse_double<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    Ok(Value::Double(r.read_f64::<BigEndian>()?))
}

#[cfg(feature = "amf3-string")]
fn parse_string<R>(_r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    unimplemented!()
}

#[cfg(feature = "amf3-xml_doc")]
fn parse_xml_document<R>(_r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    unimplemented!()
}

#[cfg(feature = "amf3-date")]
fn parse_date<R>(_r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    unimplemented!()
}

#[cfg(feature = "amf3-array")]
fn parse_array<R>(_r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    unimplemented!()
}

#[cfg(feature = "amf3-object")]
fn parse_object<R>(_r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    unimplemented!()
}

#[cfg(feature = "amf3-xml")]
fn parse_xml<R>(_r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    unimplemented!()
}

#[cfg(feature = "amf3-byte_array")]
fn parse_byte_array<R>(_r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    unimplemented!()
}

#[cfg(feature = "amf3-vector_int")]
fn parse_vector_int<R>(_r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    unimplemented!()
}

#[cfg(feature = "amf3-vector_uint")]
fn parse_vector_uint<R>(_r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    unimplemented!()
}

#[cfg(feature = "amf3-vector_double")]
fn parse_vector_double<R>(_r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    unimplemented!()
}

#[cfg(feature = "amf3-vector_object")]
fn parse_vector_object<R>(_r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    unimplemented!()
}

#[cfg(feature = "amf3-dictionary")]
fn parse_dictionary<R>(_r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    unimplemented!()
}

fn parse_u29<R>(r: &mut R) -> Result<u32, Error>
where
    R: Read,
{
    let mut rst: u32 = 0;

    for _ in 0..3 {
        let b_i = r.read_i8()?;
        rst = (rst << 7) | (0x7F & (b_i as u32));
        if b_i >= 0 {
            return Ok(rst);
        }
    }

    let b_4 = r.read_u8()?;
    rst = (rst << 8) | (b_4 as u32);
    Ok(rst)
}
