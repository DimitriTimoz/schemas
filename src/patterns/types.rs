/// This enum contains all the types that can be used in a pattern.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Types {
    #[cfg(feature = "pattern_variant_feature")] PatternVariant(PatternVariant),
} 

impl Types {
    /// Create a new type from a string.
    pub fn from_lc_ty(lc_ty: &str) -> Option<Self> {
        match lc_ty {
            #[cfg(feature = "pattern_variant_feature")] "pattern_prop_ty_lc" => Some(Self::PatternVariant(PatternVariant::new())),
            _ => None,
        }
    }

    pub fn from_ty(ty: &str) -> Option<Self> {
        Self::from_lc_ty(&ty.to_lowercase())
    }

    /// Get the name of the type.
    pub fn lc_ty(&self) -> String {
        match self {
            #[cfg(feature = "pattern_variant_feature")] Self::PatternVariant(_) => String::from("pattern_prop_ty_lc"),
        }
    }
}
