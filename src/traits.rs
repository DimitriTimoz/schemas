use crate::prelude::*;

pub trait Schema {
    fn new() -> Self;
    fn has_lc_property(name_lc: &str) -> bool { false }
    fn has_property(name: &str) -> bool {
        Self::has_lc_property(&name.to_lowercase())
    }
    fn add_lc_property(&mut self, name_lc: &str, value: String) -> Result<(), Error>;
    fn add_property(&mut self, name: &str, value: String) -> Result<(), Error> {
        self.add_lc_property(&name.to_lowercase(), value)
    }
    fn add_lc_item(&mut self, name: &str, item: Types) -> Result<(), Error>;
    fn add_item(&mut self, name: &str, item: Types) -> Result<(), Error> {
        self.add_lc_item(&name.to_lowercase(), item)
    }
}

pub enum Error {
    InvalidProperty,
    InvalidType,
    InvalidValue,
}
