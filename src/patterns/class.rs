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
    ty: String,
    properties: HashMap<String, Vec<SchemaValue>>,
}

/// A trait implemented on [PatternType]s and its subclasses.
#[cfg(feature = "pattern_feature")]
pub trait PatternTypeTrait: Schema {
    #[cfg(feature = "pattern_prop_feature")] fn pattern_property(&self) -> Option<PatternPropertyProp> { for value in self.property_lc("pattern_prop_ty_lc") {if let Ok(prop) = value.try_into() {return Some(prop);}}None }
    
    #[cfg(feature = "pattern_prop_feature")] fn take_pattern_property(&mut self) -> Option<Vec<PatternPropertyProp>>  { self.take_property_lc("pattern_prop_ty_lc").map(|values| {values.into_iter().filter_map(|value| value.try_into().ok()).collect()}) }
    
    #[cfg(feature = "pattern_prop_feature")] fn pattern_property_vec(&self) -> Vec<PatternPropertyProp> { let mut vec = Vec::new();for value in self.property_lc("pattern_prop_ty_lc") {if let Ok(prop) = value.try_into() {vec.push(prop);}}vec }

    #[cfg(feature = "pattern_prop_feature")]  fn set_pattern_property(&mut self, value: impl Into<PatternPropertyProp> + Into<SchemaValue>) { self.set_property_lc("pattern_prop_ty_lc", vec![value.into()]); }

    #[cfg(feature = "pattern_prop_feature")]  fn set_pattern_property_vec(&mut self, values: impl IntoIterator<Item = impl Into<PatternPropertyProp> + Into<SchemaValue>>) { self.set_property_lc("pattern_prop_ty_lc", values.into_iter().map(|value| value.into()).collect()); }
}

#[cfg(all(feature = "pattern_feature", feature = "PatternParent"))] impl PatternParentTrait for PatternType {}

#[cfg(feature = "pattern_feature")]
impl Schema for PatternType {
    fn new() -> PatternType {
        PatternType {
            ty: String::from("pattern_ty_lc"),
            properties: HashMap::new()
        }
    }

    fn property_lc(&self, lc_name: &str) -> &Vec<SchemaValue> {
        self.properties.get(lc_name).unwrap_or(&EMPTY_VEC)
    }

    fn set_property_lc(&mut self, lc_name: &str, values: Vec<SchemaValue>) {
        self.properties.insert(lc_name.to_string(), values);
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
            ].contains(&value.ty.as_str())
        {
            return Err(());
        }

        Ok(PatternType { ty: value.ty, properties: value.properties })
    }
}

#[cfg(feature = "pattern_feature")]
impl From<PatternType> for SchemaObject {
    fn from(value: PatternType) -> SchemaObject {
        SchemaObject { ty: value.ty, properties: value.properties }
    }
}
