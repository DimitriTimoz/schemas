use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Text(String);

impl Schema for Text {
    fn new() -> Self {
        Self(String::new())
    }

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.as_str() {
            "text" => Ok(self.0 = value),
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.as_str() {
            "text" => match item {
                Types::Text(text) => Ok(self.0 = text.0),
                _ => Err(Error::InvalidType),
            },
            _ => Err(Error::InvalidProperty),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Number(f64);

impl Schema for Number {
    fn new() -> Self {
        Self(0.0)
    }

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.as_str() {
            "number" => match value.parse::<f64>() {
                Ok(number) => Ok(self.0 = number),
                Err(_) => Err(Error::InvalidValue),
            },
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.as_str() {
            "number" => match item {
                Types::Number(number) => Ok(self.0 = number.0),
                _ => Err(Error::InvalidType),
            },
            _ => Err(Error::InvalidProperty),
        }
    }
}
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Integer(i64);
impl Schema for Integer {
    fn new() -> Self {
        Self(0)
    }

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.as_str() {
            "integer" => match value.parse::<i64>() {
                Ok(number) => Ok(self.0 = number),
                Err(_) => Err(Error::InvalidValue),
            },
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.as_str() {
            "integer" => match item {
                Types::Integer(integer) => Ok(self.0 = integer.0),
                _ => Err(Error::InvalidType),
            },
            _ => Err(Error::InvalidProperty),
        }
    }
}
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Boolean(bool);

impl Boolean {
    pub fn new() -> Self {
        Self(false)
    }
}   

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Date(String);

impl Date {
    pub fn new() -> Self {
        Self(String::new())
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DateTime(String);

impl DateTime {
    pub fn new() -> Self {
        Self(String::new())
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct URL(Text);

impl URL {
    pub fn new() -> Self {
        Self(Text::new())
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Time(String);

impl Time {
    pub fn new() -> Self {
        Self(String::new())
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct XPathType {
    sub_class: Text,
}


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CssSelectorType {
    sub_class: Text,
}

include!(concat!(env!("OUT_DIR"), "/types.rs"));
