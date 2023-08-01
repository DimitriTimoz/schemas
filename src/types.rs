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
        match name.to_lowercase().as_str() {
            "text" => {
                self.0 = value;
                Ok(())
            }
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "text" => match item {
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

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "number" => match value.parse::<f64>() {
                Ok(number) => {
                    self.0 = number;
                    Ok(())
                }
                Err(_) => Err(Error::InvalidValue),
            },
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
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

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "integer" => match value.parse::<i64>() {
                Ok(number) => {
                    self.0 = number;
                    Ok(())
                }
                Err(_) => Err(Error::InvalidValue),
            },
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "integer" => match item {
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

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "boolean" => match value.parse::<bool>() {
                Ok(boolean) => {
                    self.0 = boolean;
                    Ok(())
                }
                Err(_) => Err(Error::InvalidValue),
            },
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "boolean" => match item {
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

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "date" => {
                self.0 = value.clone();
                Ok(())
            }
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "date" => match item {
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

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "datetime" => {
                self.0 = value.clone();
                Ok(())
            }
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "datetime" => match item {
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

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "url" => {
                self.0 = Text(value);
                Ok(())
            }
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "url" => match item {
                Types::URL(url) => {
                    self.0 = url.0;
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            "text" => match item {
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

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "time" => {
                self.0 = value.clone();
                Ok(())
            }
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "time" => match item {
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

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "xpathtype" => {
                self.0 = value.clone();
                Ok(())
            }
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "xpathtype" => match item {
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

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "cssselectortype" => {
                self.0 = value.clone();
                Ok(())
            }
            _ => Err(Error::InvalidProperty),
        }
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "cssselectortype" => match item {
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

include!(concat!(env!("OUT_DIR"), "/types.rs"));
