use crate::types::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A property whose only value is Text
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextOnlyProp {
    Text(Text),
}

impl From<Text> for TextOnlyProp {
    fn from(value: Text) -> Self { Self::Text(value) }
}

/// A property that's only a class or text
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SimpleProp<T: std::fmt::Debug + Clone> {
    Text(Text),
    Value(T),
}

impl<T: std::fmt::Debug + Clone> From<T> for SimpleProp<T> {
    fn from(value: T) -> Self {
        if std::any::type_name::<T>() == "Text" {
            Self::Text(value.into())
        } else {
            Self::Value(value)
        }
    }
}

include!("properties_gen.rs");
