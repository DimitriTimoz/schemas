use std::{env, fs::{File, self, OpenOptions}, io::{Write, self}, path::Path};
use writer::ToWrite;
use parse_file::read_schema;

mod parse_file;
mod writer;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap_or(String::from("generated"));
    std::fs::remove_dir_all(&out_dir).unwrap();
    copy_dir_all("base", &out_dir).unwrap();

    let mut table = read_schema();

    // Write properties in file
    let (props, classes, features, types) = ToWrite::write_files(&mut table);

    let mut file = File::create(Path::new(&out_dir).join("src/properties_gen.rs")).unwrap();
    file.write_all(props.as_bytes()).unwrap();

    let mut file = File::create(Path::new(&out_dir).join("src/classes_gen.rs")).unwrap();
    file.write_all(classes.as_bytes()).unwrap();

    let mut file = OpenOptions::new().write(true).append(true).open(Path::new(&out_dir).join("Cargo.toml")).unwrap();
    file.write_all(features.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();

    let mut file = File::create(Path::new(&out_dir).join("src/types_gen.rs")).unwrap();
    file.write_all(types.as_bytes()).unwrap();
}

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SchemaValue {
    Boolean(bool),
    Text(String),
    Object(SchemaObject),
}

#[derive(Debug, Clone)]
pub struct SchemaObject {
    ty: String,
    properties: HashMap<String, Vec<SchemaValue>>,
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

#[derive(Debug, Clone, Default, )]
pub struct ImageObject {
    properties: HashMap<String, Vec<SchemaValue>>,
}

impl ImageObject {
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

    pub fn representative_of_page(&self) -> Option<RepresentativeOfPageProp> {
        for value in self.property("representativeofpage") {
            if let Ok(prop) = value.try_into() {
                return Some(prop);
            }
        }
        None
    }
    pub fn take_representative_of_page(&mut self) -> Option<Vec<RepresentativeOfPageProp>> {
        self.take_property("representativeofpage").map(|values| {
            values.into_iter().filter_map(|value| value.try_into().ok()).collect()
        })
    }
    pub fn representative_of_page_vec(&self) -> Vec<RepresentativeOfPageProp> {
        let mut vec = Vec::new();
        for value in self.property("representativeofpage") {
            if let Ok(prop) = value.try_into() {
                vec.push(prop);
            }
        }
        vec
    }
}

impl TryFrom<SchemaObject> for ImageObject {
    type Error = ();

    fn try_from(value: SchemaObject) -> Result<Self, Self::Error> {
        if value.ty != "image" {
            return Err(());
        }

        Ok(ImageObject { properties: value.properties })
    }
}

#[derive(Debug, Clone, )]
pub enum RepresentativeOfPageProp {
    Boolean(bool),
    Text(String),
}

impl TryFrom<SchemaValue> for RepresentativeOfPageProp {
    type Error = ();

    fn try_from(value: SchemaValue) -> Result<Self, Self::Error> {
        match value {
            SchemaValue::Boolean(b) => Ok(RepresentativeOfPageProp::Boolean(b)),
            SchemaValue::Text(s) => Ok(RepresentativeOfPageProp::Text(s)),
            _ => Err(()),
        }
    }
}

impl TryFrom<&SchemaValue> for RepresentativeOfPageProp {
    type Error = ();

    fn try_from(value: &SchemaValue) -> Result<Self, Self::Error> {
        match value {
            SchemaValue::Boolean(b) => Ok(RepresentativeOfPageProp::Boolean(b.clone())),
            SchemaValue::Text(s) => Ok(RepresentativeOfPageProp::Text(s.clone())),
            _ => Err(()),
        }
    }
}

