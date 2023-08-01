use crate::properties::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Text(String);

impl Schema for Text {
    fn new() -> Self {
        Self(String::new())
    }

    fn add_property(&mut self, name: String, value: String) {
        match name.as_str() {
            "text" => self.0 = value,
            _ => (),
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

    fn add_property(&mut self, name: String, value: String) {
        match name.as_str() {
            "number" => self.0 = value.parse::<f64>().unwrap(),
            _ => (),
        }
    }
}
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Integer(i64);

impl Integer {
    pub fn new() -> Self {
        Self(0)
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
