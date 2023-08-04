/// PatternDoc
#[derive()]
pub struct PatternType {
    pub multi_pattern_property: Vec<MultiPatternProperty>,
    pub sub_class: SubClass,
}

impl Schema for PatternType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
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
