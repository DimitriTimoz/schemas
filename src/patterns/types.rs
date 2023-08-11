/// This enum contains all the types that can be used in a pattern.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Types {
    Text(Text),
    Number(Number),
    Integer(Integer),
    Boolean(Boolean),
    Date(Date),
    DateTime(DateTime),
    Url(Url),
    Time(Time),
    XPathType(XPathType),
    CssSelectorType(CssSelectorType),
    #[cfg(feature = "pattern_variant_feature")] PatternVariant(PatternVariant),
} 

impl Types {
    /// Create a new type from a string.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "text" => Some(Self::Text(Text::new())),
            "number" => Some(Self::Number(Number::new())),
            "integer" => Some(Self::Integer(Integer::new())),
            "boolean" => Some(Self::Boolean(Boolean::new())),
            "date" => Some(Self::Date(Date::new())),
            "datetime" => Some(Self::DateTime(DateTime::new())),
            "url" => Some(Self::Url(Url::new())),
            "time" => Some(Self::Time(Time::new())),
            "xpathtype" => Some(Self::XPathType(XPathType::new())),
            "cssselectortype" => Some(Self::CssSelectorType(CssSelectorType::new())),
            #[cfg(feature = "pattern_variant_feature")] "pattern_prop_name_lc" => Some(Self::PatternVariant(PatternVariant::new())),
            _ => None,
        }
    }

    /// Get the name of the type.
    pub fn name(&self) -> String {
        match self {
            Types::Text(_) => String::from("text"),
            Types::Number(_) => String::from("number"),
            Types::Integer(_) => String::from("integer"),
            Types::Boolean(_) => String::from("boolean"),
            Types::Date(_) => String::from("date"),
            Types::DateTime(_) => String::from("datetime"),
            Types::Url(_) => String::from("url"),
            Types::Time(_) => String::from("time"),
            Types::XPathType(_) => String::from("xpathtype"),
            Types::CssSelectorType(_) => String::from("cssselectortype"),
            #[cfg(feature = "pattern_variant_feature")] Self::PatternVariant(_) => String::from("pattern_prop_name_lc"),
        }
    }
}
