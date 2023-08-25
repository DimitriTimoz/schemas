use crate::prelude::*;

pub trait Schema {
    fn new() -> Self;

    fn properties(&self) -> &HashMap<String, Vec<SchemaValue>>;
    fn properties_mut(&mut self) -> &mut HashMap<String, Vec<SchemaValue>>;

    fn property_lc(&self, lc_name: &str) -> &Vec<SchemaValue> {
        self.properties().get(lc_name).unwrap_or(&EMPTY_VEC)
    }
    fn property(&self, name: &str) -> &Vec<SchemaValue> {
        self.property_lc(&name.to_lowercase())
    }

    fn set_property_lc(&mut self, lc_name: &str, values: Vec<SchemaValue>) {
        self.properties_mut().insert(lc_name.to_string(), values)
    }
    fn set_property(&mut self, name: &str, values: Vec<SchemaValue>) {
        self.set_property_lc(&name.to_lowercase(), values);
    }

    fn take_property_lc(&mut self, lc_name: &str) -> Option<Vec<SchemaValue>> {
        self.properties_mut().remove(lc_name)
    }
    fn take_property(&mut self, name: &str) -> Option<Vec<SchemaValue>> {
        self.take_property_lc(&name.to_lowercase())
    }
}

pub enum Error {
    InvalidProperty,
    InvalidType,
    InvalidValue,
}
