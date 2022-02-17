pub mod amf3;
pub mod amf0;

#[derive(Debug)]
pub enum Value {
    Amf3Value(amf3::Value),
    Amf0Value(amf0::Value),
}
