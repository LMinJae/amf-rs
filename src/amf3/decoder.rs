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
        marker::INTEGER => parse_integer(r),
        marker::DOUBLE => parse_double(r),
        _ => Err(Error::new(ErrorKind::Other, "Unsupported marker")),
    }
}

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

fn parse_double<R>(r: &mut R) -> Result<Value, Error>
where
    R: Read,
{
    Ok(Value::Double(r.read_f64::<BigEndian>()?))
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
