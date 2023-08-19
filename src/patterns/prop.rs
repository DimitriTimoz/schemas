/// PatternDoc
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg(feature = "pattern_feature")]
pub enum PatternTypeProp {
    PatternVariant(PatternInnerVariant),
}

#[automatically_derived]
#[cfg(feature = "pattern_feature")]
impl TryFrom<SchemaValue> for PatternTypeProp {
    type Error = ();

    fn try_from(value: SchemaValue) -> Result<Self, Self::Error> {
        match value {
            SchemaValue::PatternPrimitiveVariant(v) => Ok(PatternTypeProp::PatternSecondPrimitiveVariant(v)),
            SchemaValue::Object(v) => {
                // TODO: move clone away
                if let Ok(object) = PatternObjectVariant::try_from(v.clone()) { return Ok(PatternTypeProp::PatternObjectVariant(object)) }
                Err(())
            },
            _ => Err(()),
        }
    }
}

#[automatically_derived]
#[cfg(feature = "pattern_feature")]
impl TryFrom<&SchemaValue> for PatternTypeProp {
    type Error = ();

    fn try_from(value: &SchemaValue) -> Result<Self, Self::Error> {
        match value {
            SchemaValue::PatternPrimitiveVariant(v) => Ok(PatternTypeProp::PatternSecondPrimitiveVariant(v.clone())),
            SchemaValue::Object(v) => {
                // TODO: move clone away
                if let Ok(object) = PatternObjectVariant::try_from(v.clone()) { return Ok(PatternTypeProp::PatternObjectVariant(object)) }
                Err(())
            },
            _ => Err(()),
        }
    }
}

#[automatically_derived]
#[cfg(feature = "pattern_feature")]
impl From<PatternTypeProp> for SchemaValue {
    fn from(value: PatternTypeProp) -> Self {
        match value {
            PatternTypeProp::PatternSecondPrimitiveVariant(v) => SchemaValue::PatternPrimitiveVariant(v),
            PatternTypeProp::PatternObjectVariant(v) => SchemaValue::Object(v.into()),
        }
    }
}
