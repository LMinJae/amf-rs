use std::io::{Error, ErrorKind, Read};
use std::collections::HashMap;
use byteorder::{BigEndian, ReadBytesExt};

use super::{marker, Value};

pub fn from_bytes<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    let marker = r.read_u8()?;
    match marker {
        #[cfg(feature = "amf0-number")]
        marker::NUMBER          => parse_number(r),
        #[cfg(feature = "amf0-boolean")]
        marker::BOOLEAN         => parse_boolean(r),
        #[cfg(feature = "amf0-string")]
        marker::STRING          => parse_string(r),
        #[cfg(feature = "amf0-object")]
        marker::OBJECT          => parse_object(r),
        marker::NULL            => Ok(Value::Null),
        marker::UNDEFINED       => Ok(Value::Undefined),
        #[cfg(feature = "amf0-reference")]
        marker::REFERENCE       => parse_reference(r),
        #[cfg(feature = "amf0-ecma_array")]
        marker::ECMA_ARRAY      => parse_ecma_array(r),
        #[cfg(feature = "amf0-object")]
        marker::OBJECT_END       => Ok(Value::ObjectEnd),
        #[cfg(feature = "amf0-strict_array")]
        marker::STRICT_ARRAY    => parse_strict_array(r),
        #[cfg(feature = "amf0-date")]
        marker::DATE            => parse_date(r),
        #[cfg(feature = "amf0-string")]
        marker::LONG_STRING     => parse_long_string(r),
        #[cfg(feature = "amf0-xml_document")]
        marker::XML_DOCUMENT    => parse_xml_document(r),
        #[cfg(feature = "amf0-object")]
        marker::TYPED_OBJECT    => parse_typed_object(r),
        _ => Ok(Value::Unsupported)
    }
}

#[cfg(feature = "amf0-number")]
fn parse_number<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    Ok(Value::Number(r.read_f64::<BigEndian>()?))
}

#[cfg(feature = "amf0-boolean")]
fn parse_boolean<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    Ok(Value::Boolean(0 != (r.read_u8()?)))
}

#[cfg(feature = "amf0-string")]
fn parse_string<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    let len = r.read_u16::<BigEndian>()? as usize;
    
    let mut buf = vec![0; len];
    r.read_exact(&mut buf)?;
    if let Ok(utf8) = String::from_utf8(buf) {
        return Ok(Value::String(utf8))
    }

    Err(Error::new(ErrorKind::Other, "Failed to parsing UTF-8"))
}

#[cfg(feature = "amf0-object")]
fn parse_object<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    match parse_object_property(r) {
        Ok(property) => Ok(Value::Object { class_name: "".to_string(), property: property}),
        Err(e) => Err(e),
    }
}

#[cfg(feature = "amf0-reference")]
fn parse_reference<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    Ok(Value::Reference(r.read_u16::<BigEndian>()?))
}

#[cfg(feature = "amf0-ecma_array")]
fn parse_ecma_array<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    // Unsafe: read associative-count, but not used
    let _len = r.read_u32::<BigEndian>()? as usize;
    match parse_object_property(r) {
        Ok(property) => Ok(Value::ECMAArray(property)),
        Err(e) => Err(e),
    }
}

#[cfg(feature = "amf0-strict_array")]
fn parse_strict_array<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    let len = r.read_u32::<BigEndian>()? as usize;
    let mut vec = Vec::<Value>::new();

    for _ in 0..len {
        vec.push(from_bytes(r)?);
    }
    
    Ok(Value::StrictArray(vec))
}

#[cfg(feature = "amf0-date")]
fn parse_date<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    Ok(Value::Date(r.read_f64::<BigEndian>()?))
}

#[cfg(feature = "amf0-string")]
fn parse_long_string<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    let len = r.read_u32::<BigEndian>()? as usize;
    
    let mut buf = vec![0; len];
    r.read_exact(&mut buf)?;
    if let Ok(utf8) = String::from_utf8(buf) {
        return Ok(Value::String(utf8))
    }

    Err(Error::new(ErrorKind::Other, "Failed to parsing UTF-8"))
}

#[cfg(feature = "amf0-xml_document")]
fn parse_xml_document<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    let len = r.read_u32::<BigEndian>()? as usize;
    
    let mut buf = vec![0; len];
    r.read_exact(&mut buf)?;
    if let Ok(utf8) = String::from_utf8(buf) {
        return Ok(Value::XMLDocument(utf8))
    }

    Err(Error::new(ErrorKind::Other, "Failed to parsing UTF-8"))
}

#[cfg(feature = "amf0-object")]
fn parse_typed_object<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    let len = r.read_u16::<BigEndian>()? as usize;
    
    let mut buf = vec![0; len];
    r.read_exact(&mut buf)?;
    match String::from_utf8(buf) {
        Ok(class_name) => {
            match parse_object_property(r) {
                Ok(property) => Ok(Value::Object { class_name: class_name, property: property}),
                Err(e) => Err(e),
            }
        },
        _ => Err(Error::new(ErrorKind::Other, "Failed to parsing UTF-8"))
    }
}

#[cfg(any(feature = "amf0-object", feature = "amf0-ecma_array"))]
fn parse_object_property<R>(r: &mut R) -> Result<HashMap<String, Value>, Error>
where
    R: Read,
{
    let mut property = HashMap::new();
    loop {
        let len = r.read_u16::<BigEndian>()? as usize;
        let key = if 0 == len {
            "".to_string()
        } else {
            let mut buf = vec![0; len];
            r.read_exact(&mut buf)?;
            match String::from_utf8(buf) {
                Ok(utf8) => utf8,
                _ => return Err(Error::new(ErrorKind::Other, "Failed to parsing value"))
            }
        };

        match from_bytes(r) {
            Ok(Value::ObjectEnd) => return Ok(property),
            Ok(value) => property.insert(key, value),
            _ => return Err(Error::new(ErrorKind::Other, "Failed to parsing UTF-8"))
        };
    }
}
