use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SchemaValue {
    Text(String),
    Url(String),
    XPath(String),
    CssSelector(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Date(String),
    Time(String),
    DateTime(String),
    Object(SchemaObject),
}

#[derive(Debug, Clone)]
pub struct SchemaObject {
    pub(crate) ty: String,
    pub(crate) properties: HashMap<String, Vec<SchemaValue>>,
}

static EMPTY_VEC: Vec<SchemaValue> = Vec::new();

impl SchemaObject {
    pub fn new_lc_ty(lc_ty: String) -> SchemaObject {
        SchemaObject {
            ty: lc_ty,
            properties: HashMap::new(),
        }
    }

    pub fn new(ty: String) -> SchemaObject {
        SchemaObject::new_lc_ty(ty.to_lowercase())
    }

    pub fn add_lc_property(&mut self, lc_name: String, value: SchemaValue) {
        self.properties.entry(lc_name).or_insert(Vec::new()).push(value);
    }

    pub fn add_property(&mut self, name: String, value: SchemaValue) {
        self.add_lc_property(name.to_lowercase(), value);
    }

    pub fn property_lc(&self, lc_name: &str) -> &Vec<SchemaValue> {
        self.properties.get(lc_name).unwrap_or(&EMPTY_VEC)
    }

    pub fn property(&self, name: &str) -> &Vec<SchemaValue> {
        self.property_lc(&name.to_lowercase())
    }

    pub fn take_property_lc(&mut self, lc_name: &str) -> Option<Vec<SchemaValue>> {
        self.properties.remove(lc_name)
    }

    pub fn take_property(&mut self, name: &str) -> Option<Vec<SchemaValue>> {
        self.take_property_lc(&name.to_lowercase())
    }
}
