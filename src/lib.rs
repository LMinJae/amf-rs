pub mod amf0;
pub mod amf3;

#[cfg(test)]
mod tests;

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
