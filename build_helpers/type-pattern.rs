/// PatternDoc
#[derive(
    Debug,
    Clone,
    Default,
    PatternDerive,
)]
pub struct PatternType {
    pub pattern_property: Vec<PatternPropertyProp>,
    pub pattern_parent: PatternParent,
}

impl Schema for PatternType {
    fn new() -> Self {
        Self::default()
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "pattern_prop_name" => self.pattern_property.push(PatternPropertyProp::Text(value)),
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}
