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

    fn add_lc_property(&mut self, name: &str, value: Types) -> Result<(), Error> {
        match name {
            "text" => match value {
                Types::Text(text) => {
                    self.0 = text.0;
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => Err(Error::InvalidProperty),
        }
    }
}

impl From<String> for Text {
    fn from(text: String) -> Self {
        Self(text)
    }
}

impl From<&str> for Text {
    fn from(text: &str) -> Self {
        Self(text.to_string())
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Number(f64);

impl Schema for Number {
    fn new() -> Self {
        Self(0.0)
    }

    fn add_lc_property(&mut self, name: &str, item: Types) -> Result<(), Error> {
        match name {
            "number" => match item {
                Types::Number(number) => {
                    self.0 = number.0;
                    Ok(())
                }
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

    fn add_lc_property(&mut self, name: &str, value: Types) -> Result<(), Error> {
        match name {
            "integer" => match value {
                Types::Integer(integer) => {
                    self.0 = integer.0;
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => Err(Error::InvalidProperty),
        }
    }
}
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Boolean(bool);

impl Schema for Boolean {
    fn new() -> Self {
        Self(false)
    }

    fn add_lc_property(&mut self, name: &str, value: Types) -> Result<(), Error> {
        match name {
            "boolean" => match value {
                Types::Boolean(boolean) => {
                    self.0 = boolean.0;
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => Err(Error::InvalidProperty),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Date(String);

impl Schema for Date {
    fn new() -> Self {
        Self(String::new())
    }

    fn add_lc_property(&mut self, name: &str, value: Types) -> Result<(), Error> {
        match name {
            "date" => match value {
                Types::Date(date) => {
                    self.0 = date.0;
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => Err(Error::InvalidProperty),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DateTime(String);

impl Schema for DateTime {
    fn new() -> Self {
        Self(String::new())
    }

    fn add_lc_property(&mut self, name: &str, value: Types) -> Result<(), Error> {
        match name {
            "datetime" => match value {
                Types::Date(date) => {
                    self.0 = date.0;
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => Err(Error::InvalidProperty),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct URL(Text);

impl From<String> for URL {
    fn from(text: String) -> Self {
        Self(Text(text))
    }
}

impl From<&str> for URL {
    fn from(text: &str) -> Self {
        Self(Text(text.to_string()))
    }
}

impl Schema for URL {
    fn new() -> Self {
        Self(Text::new())
    }

    fn add_lc_property(&mut self, name: &str, value: Types) -> Result<(), Error> {
        match name {
            "url" => match value {
                Types::URL(url) => {
                    self.0 = url.0;
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            "text" => match value {
                Types::Text(text) => {
                    self.0 = text;
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => Err(Error::InvalidProperty),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Time(String);

impl Schema for Time {
    fn new() -> Self {
        Self(String::new())
    }

    fn add_lc_property(&mut self, name: &str, value: Types) -> Result<(), Error> {
        match name {
            "time" => match value {
                Types::Time(date) => {
                    self.0 = date.0;
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => Err(Error::InvalidProperty),
        }
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct XPathType(String);

impl Schema for XPathType {
    fn new() -> Self {
        Self(String::new())
    }

    fn add_lc_property(&mut self, name: &str, value: Types) -> Result<(), Error> {
        match name {
            "xpathtype" => match value {
                Types::XPathType(xpathtype) => {
                    self.0 = xpathtype.0;
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => Err(Error::InvalidProperty),
        }
    }
}
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CssSelectorType(String);

impl Schema for CssSelectorType {
    fn new() -> Self {
        Self(String::new())
    }

    fn add_lc_property(&mut self, name: &str, value: Types) -> Result<(), Error> {
        match name {
            "cssselectortype" => match value {
                Types::CssSelectorType(cssselectortype) => {
                    self.0 = cssselectortype.0;
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => Err(Error::InvalidProperty),
        }
    }
}

include!("classes_gen.rs");
include!("types_gen.rs");
