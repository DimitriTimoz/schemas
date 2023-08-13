use crate::prelude::*;

pub trait Schema {
    fn new() -> Self;

    fn property_lc(&self, lc_name: &str) -> &Vec<SchemaValue>;
    fn property(&self, name: &str) -> &Vec<SchemaValue> {
        self.property_lc(&name.to_lowercase())
    }

    fn take_property_lc(&mut self, lc_name: &str) -> Option<Vec<SchemaValue>>;
    fn take_property(&mut self, name: &str) -> Option<Vec<SchemaValue>> {
        self.take_property_lc(&name.to_lowercase())
    }
}

pub enum Error {
    InvalidProperty,
    InvalidType,
    InvalidValue,
}
