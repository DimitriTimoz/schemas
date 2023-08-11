/// PatternDoc
#[derive(Debug, Clone, PatternDerive)]
#[cfg(feature = "pattern_feature")]
pub enum PatternTypeProp {
    PatternVariant(PatternVariant),
}

#[automatically_derived] #[cfg(feature = "pattern_feature")] impl From<PatternVariant> for PatternTypeProp { fn from(value: PatternVariant) -> Self { Self::PatternVariant(value) } }
