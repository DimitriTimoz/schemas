/// This enum contains all the types that can be used in a pattern.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Types {
    PatternVariant(PatternVariant),
} 

impl Types {
    /// Create a new type from a string.
    pub fn fom_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "pattern_prop_name_lc" => Some(Self::PatternVariant(PatternVariant::new())),
            _ => None,
        }
    }

    /// Get the name of the type.
    pub fn name(&self) -> String {
        match self {
            Self::PatternVariant(_) => String::from("pattern_prop_name_lc"),
        }
    }
}