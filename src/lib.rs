use core::ops::Index;
use core::hash::Hash;
use core::borrow::Borrow;

pub mod amf0;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub enum Value {
    Amf0Value(amf0::Value),
}

pub type Boolean = bool;
pub type Integer = i32;
pub type Double = f64;
pub type String = std::string::String;
pub type Property<T> = std::collections::HashMap<String, T>;
#[derive(Debug, PartialEq, Clone)]
pub struct Object<T> {
    pub class_name: String,
    pub property: Property<T>,
}
impl<Q: ?Sized, T> Index<&'_ Q> for Object<T>
where
    String: Borrow<Q>,
    Q: Eq + Hash
{
    type Output = T;

    fn index(&self, key: &'_ Q) -> &Self::Output {
        self.property.index(key)
    }
}
pub type Array<T> = Vec<T>;
