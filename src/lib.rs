pub mod amf3;
pub mod amf0;

#[derive(Debug)]
pub enum Value {
    Amf3Value(amf3::Value),
    Amf0Value(amf0::Value),
}

pub type Boolean = bool;
pub type Integer = i32;
pub type Double = f64;
pub type String = std::string::String;
pub type Property<T> = std::collections::HashMap<String, T>;
#[derive(Debug, PartialEq, Clone)]
pub struct Object<T> {
    class_name: String,
    property: Property<T>,
}
pub type Array<T> = Vec<T>;
#[derive(Debug, PartialEq, Clone)]
pub struct Date {
    millis: f64,
    timezone: u16,
}
impl Date {
    pub fn new(millis: f64) -> Self {
        Date {
            millis,
            timezone: 0,
        }
    }
}
