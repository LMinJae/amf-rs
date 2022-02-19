use byteorder::{BigEndian, WriteBytesExt};
use std::io::{Error, Write};

use super::{marker, Value};

pub fn to_bytes<W>(w: &mut W, v: &Value) -> Result<(), Error>
where
    W: Write,
{
    match v {
        #[cfg(feature = "amf0-number")]
        Value::Number(f) => {
            w.write_u8(marker::NUMBER)?;
            w.write_f64::<BigEndian>(*f)
        }
        #[cfg(feature = "amf0-boolean")]
        Value::Boolean(b) => {
            w.write_u8(marker::BOOLEAN)?;
            w.write_u8(if *b { 0xFF } else { 0 })
        }
        #[cfg(feature = "amf0-string")]
        Value::String(s) => {
            if 0xffff < s.len() {
                w.write_u8(marker::LONG_STRING)?;
                w.write_u32::<BigEndian>(s.len() as u32)?;
            } else {
                w.write_u8(marker::STRING)?;
                w.write_u16::<BigEndian>(s.len() as u16)?;
            }
            w.write_all(s.as_bytes())
        }
        #[cfg(feature = "amf0-object")]
        Value::Object(obj) => {
            if 0 == obj.class_name.len() {
                w.write_u8(marker::OBJECT)?;
            } else {
                w.write_u8(marker::TYPED_OBJECT)?;
                w.write_u16::<BigEndian>(obj.class_name.len() as u16)?;
                w.write_all(obj.class_name.as_bytes())?;
            }
            for (k, v) in obj.property.iter() {
                w.write_u16::<BigEndian>(k.len() as u16)?;
                w.write_all(k.as_bytes())?;

                to_bytes(w, v)?;
            }
            to_bytes(w, &Value::ObjectEnd)
        }
        Value::Null => w.write_u8(marker::NULL),
        Value::Undefined => w.write_u8(marker::UNDEFINED),
        #[cfg(feature = "amf0-ecma_array")]
        Value::ECMAArray(property) => {
            w.write_u8(marker::ECMA_ARRAY)?;
            w.write_u32::<BigEndian>(property.len() as u32)?;
            for (k, v) in property.iter() {
                w.write_u16::<BigEndian>(k.len() as u16)?;
                w.write_all(k.as_bytes())?;

                to_bytes(w, v)?;
            }
            to_bytes(w, &Value::ObjectEnd)
        }
        Value::ObjectEnd => {
            w.write_u16::<BigEndian>(0)?;
            w.write_u8(marker::OBJECT_END)
        }
        #[cfg(feature = "amf0-strict_array")]
        Value::StrictArray(property) => {
            w.write_u8(marker::STRICT_ARRAY)?;
            w.write_u32::<BigEndian>(property.len() as u32)?;
            for i in property.iter() {
                to_bytes(w, i)?;
            }
            to_bytes(w, &Value::ObjectEnd)
        }
        #[cfg(feature = "amf0-date")]
        Value::Date(d) => {
            w.write_u8(marker::DATE)?;
            w.write_f64::<BigEndian>(d.millis)?;
            w.write_u16::<BigEndian>(d.timezone)
        }
        #[cfg(feature = "amf0-xml_document")]
        Value::XMLDocument(xml) => {
            w.write_u8(marker::XML_DOCUMENT)?;
            w.write_u32::<BigEndian>(xml.len() as u32)?;
            w.write_all(xml.as_bytes())
        }
        #[cfg(feature = "amf0-avmplus")]
        Value::AVMPlus(_avm) => {
            w.write_u8(marker::AVMPLUS_OBJECT)?;
            unimplemented!();
        }
        _ => w.write_u8(marker::UNSUPPORTED),
    }
}
