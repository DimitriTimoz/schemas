use crate::prelude::*;

pub trait Schema {
    fn new() -> Self;
    fn has_lc_property(_name_lc: &str) -> bool { false }
    fn has_property(name: &str) -> bool {
        Self::has_lc_property(&name.to_lowercase())
    }
    fn add_lc_property(&mut self, property_lc: &str, value: Types) -> Result<(), Error>;
    fn add_property(&mut self, property: &str, value: Types) -> Result<(), Error> {
        self.add_lc_property(&property.to_lowercase(), value)
    }
    fn add_text_propery(&mut self, property: &str, value: String) -> Result<(), Error> {
        self.add_lc_property(&property.to_lowercase(), Types::Text(value.into()))
    }
}

pub enum Error {
    InvalidProperty,
    InvalidType,
    InvalidValue,
}
