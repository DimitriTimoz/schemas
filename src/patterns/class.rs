/// PatternDoc
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg(feature = "pattern_feature")]
pub struct PatternType {
    properties: HashMap<String, Vec<SchemaValue>>,
}

#[automatically_derived]
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

#[automatically_derived]
#[cfg(feature = "pattern_feature")]
impl PatternType {
    #[cfg(feature = "pattern_prop_feature")] pub fn pattern_property(&self) -> Option<PatternPropertyProp> { for value in self.property("pattern_prop_ty_lc") {if let Ok(prop) = value.try_into() {return Some(prop);}}None }
    
    #[cfg(feature = "pattern_prop_feature")] pub fn take_pattern_property(&mut self) -> Option<Vec<PatternPropertyProp>> { self.take_property("pattern_prop_ty_lc").map(|values| {values.into_iter().filter_map(|value| value.try_into().ok()).collect()}) }
    
    #[cfg(feature = "pattern_prop_feature")] pub fn pattern_property_vec(&self) -> Vec<PatternPropertyProp> { let mut vec = Vec::new();for value in self.property("pattern_prop_ty_lc") {if let Ok(prop) = value.try_into() {vec.push(prop);}}vec }
}

#[automatically_derived]
#[cfg(feature = "pattern_feature")]
impl TryFrom<SchemaObject> for PatternType {
    type Error = ();

    fn try_from(value: SchemaObject) -> Result<Self, Self::Error> {
        if value.ty != "pattern_ty_lc" {
            return Err(());
        }

        Ok(PatternType { properties: value.properties })
    }
}
