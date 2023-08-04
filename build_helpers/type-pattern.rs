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

    fn has_lc_property(name: &str) -> bool {
        [
            "pattern_prop_name_lc",
        ]
        .contains(&name)
        || PatternParent::has_lc_property(name)
    }

    fn add_lc_property(&mut self, name: &str, value: String) -> Result<(), Error> {
        match name {
            "pattern_prop_name_lc" => self.pattern_property.push(PatternPropertyProp::Text(value)),
            _ => {
                if PatternParent::has_lc_property(name) { return self.pattern_parent.add_property(name, value); }
                return Err(Error::InvalidProperty);
            },
        }
        Ok(())
    }

    fn add_lc_item(&mut self, name: &str, item: Types) -> Result<(), Error> {
        match name {
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
