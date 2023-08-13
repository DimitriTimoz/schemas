/// PatternDoc
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg(feature = "pattern_feature")]
pub enum PatternTypeProp {
    PatternVariant(PatternInnerVariant),
}

#[automatically_derived]
#[cfg(feature = "pattern_feature")]
impl TryFrom<SchemaValue> for RepresentativeOfPageProp {
    type Error = ();

    fn try_from(value: SchemaValue) -> Result<Self, Self::Error> {
        match value {
            SchemaValue::PatternPrimitiveVariant(v) => Ok(RepresentativeOfPageProp::PatternPrimitiveVariant(v)),
            SchemaValue::Object(v) => {
                if let Ok(object) = PatternObjectVariant::try_from(v) { return Ok(RepresentativeOfPageProp::PatternObjectVariant(object)) }
                Err(())
            },
            _ => Err(()),
        }
    }
}

#[automatically_derived]
#[cfg(feature = "pattern_feature")]
impl TryFrom<&SchemaValue> for RepresentativeOfPageProp {
    type Error = ();

    fn try_from(value: &SchemaValue) -> Result<Self, Self::Error> {
        match value {
            SchemaValue::PatternPrimitiveVariant(v) => Ok(RepresentativeOfPageProp::PatternPrimitiveVariant(v.clone())),
            SchemaValue::Object(v) => {
                // TODO: move clone away
                if let Ok(object) = PatternObjectVariant::try_from(v.clone()) { return Ok(RepresentativeOfPageProp::PatternObjectVariant(object)) }
                Err(())
            },
            _ => Err(()),
        }
    }
}
