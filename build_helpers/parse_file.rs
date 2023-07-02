use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SchemaItemTypesCsv {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) comment: String,
    pub(crate) subTypeOf: String,
    pub(crate) enumerationtype: Option<String>,
    pub(crate) equivalentClass: Option<String>,
    pub(crate) properties: String,
    pub(crate) subTypes: Option<String>,
    pub(crate) supersedes: Option<String>,
    pub(crate) supersededBy: Option<String>,
    pub(crate) isPartOf: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SchemaItemPropertieCsv {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) comment: String,
    pub(crate) subPropertyOf: String,
    pub(crate) equivalentProperty: Option<String>,
    pub(crate) subproperties: Option<String>,
    pub(crate) domainIncludes: String,
    pub(crate) rangeIncludes: Option<String>,
    pub(crate) inverseOf: Option<String>,
    pub(crate) supersedes: Option<String>,
    pub(crate) supersededBy: String,
    pub(crate) isPartOf: Option<String>
}


use csv::Reader;

use super::writer::ToWrite;

pub struct SchemaItemType {
    pub(crate) label: String,
    pub(crate) id: String,
    pub(crate) comment: String,
    pub(crate) properties: Vec<String>,
    pub(crate) same_name: Vec<String>,
}   

pub struct SchemaItemProperty {
    pub(crate) label: String,
    pub(crate) id: String,
    pub(crate) comment: String,
    pub(crate) sub_properties: Vec<String>,
    pub(crate) same_name: Vec<String>,
    pub(crate) domain: Vec<String>
}


pub fn read_csv_schema() -> ToWrite {
    let mut to_write = ToWrite::new();

    let mut reader = Reader::from_path("schemas-types.csv").unwrap();

    for result in reader.deserialize() {
        let record: SchemaItemTypesCsv = result.unwrap();
        to_write.add_type(record.clone());
    }

    let mut reader = Reader::from_path("schemas-properties.csv").unwrap();

    for result in reader.deserialize() {
        let record: SchemaItemPropertieCsv = result.unwrap();
        to_write.add_property(record.clone());
    }
    to_write
    
}