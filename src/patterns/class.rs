/// PatternDoc
/// 
/// Contains the following properties (enable them for helper accessors):
/// - `pattern_property` ([PatternPropertyProp])
/// 
/// Descends from the following classes (enable them for conversion traits):
/// - [`PatternParent`](PatternParentTrait)
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg(feature = "pattern_feature")]
pub struct PatternType {
    properties: HashMap<String, Vec<SchemaValue>>,
}

/// A trait implemented on [PatternType]s and its subclasses.
#[cfg(feature = "pattern_feature")]
pub trait PatternTypeTrait: Schema {
    #[cfg(feature = "pattern_prop_feature")] fn pattern_property(&self) -> Option<PatternPropertyProp> { for value in self.property("pattern_prop_ty_lc") {if let Ok(prop) = value.try_into() {return Some(prop);}}None }
    
    #[cfg(feature = "pattern_prop_feature")] fn take_pattern_property(&mut self) -> Option<Vec<PatternPropertyProp>>  { self.take_property("pattern_prop_ty_lc").map(|values| {values.into_iter().filter_map(|value| value.try_into().ok()).collect()}) }
    
    #[cfg(feature = "pattern_prop_feature")] fn pattern_property_vec(&self) -> Vec<PatternPropertyProp> { let mut vec = Vec::new();for value in self.property("pattern_prop_ty_lc") {if let Ok(prop) = value.try_into() {vec.push(prop);}}vec }
}

#[cfg(all(feature = "pattern_feature", feature = "PatternParent"))] impl PatternParentTrait for PatternType {}

#[cfg(feature = "pattern_feature")]
impl Schema for PatternType {
    fn new() -> PatternType {
        PatternType { properties: HashMap::new() }
    }

    fn property_lc(&self, lc_name: &str) -> &Vec<SchemaValue> {
        self.properties.get(lc_name).unwrap_or(&EMPTY_VEC)
    }

    fn take_property_lc(&mut self, lc_name: &str) -> Option<Vec<SchemaValue>> {
        self.properties.remove(lc_name)
    }
}

#[cfg(feature = "pattern_feature")]
impl TryFrom<SchemaObject> for PatternType {
    type Error = ();

    fn try_from(value: SchemaObject) -> Result<Self, Self::Error> {
        if ![
                "pattern_child_ty_lc",
            ].contains(value.ty)
        {
            return Err(());
        }

        Ok(PatternType { properties: value.properties })
    }
}
