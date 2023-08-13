use crate::prelude::*;

/// A property whose only value is Text
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextOnlyProp {
    Text(Text),
}

impl From<Text> for TextOnlyProp {
    fn from(value: Text) -> Self { Self::Text(value) }
}

include!("properties_gen.rs");
