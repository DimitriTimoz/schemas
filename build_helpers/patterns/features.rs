/// PatternDoc
#[derive(Debug, Clone, PatternDerive)]
pub enum PatternTypeProp {
    PatternVariant(PatternVariant),
}

#[automatically_derived]
impl From<PatternVariant> for PatternTypeProp { fn from(value: PatternVariant) -> Self { Self::PatternVariant(value) } }
