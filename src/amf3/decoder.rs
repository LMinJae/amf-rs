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
        _ => Err(Error::new(ErrorKind::Other, "Unsupported marker")),
    }
}
