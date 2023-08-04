/// PatternDoc
#[derive(Debug, Clone, PatternDerive)]
pub enum PatternTypeProp {
    PatternVariant(PatternVariant),
}
