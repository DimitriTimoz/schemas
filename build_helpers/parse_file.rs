use std::collections::HashMap;
use std::sync::Arc;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct SchemaItemTypesCsv {
    id: String,
    label: String,
    comment: String,
    subTypeOf: String,
    enumerationtype: Option<String>,
    equivalentClass: Option<String>,
    properties: String,
    subTypes: Option<String>,
    supersedes: Option<String>,
    supersededBy: Option<String>,
    isPartOf: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SchemaItemPropertieCsv {
    id: String,
    label: String,
    comment: String,
    subPropertyOf: String,
    equivalentProperty: Option<String>,
    subproperties: Option<String>,
    domainIncludes: String,
    rangeIncludes: Option<String>,
    inverseOf: Option<String>,
    supersedes: Option<String>,
    supersededBy: String,
    isPartOf: Option<String>
}


use csv::Reader;

struct SchemaItemType {
    label: String,
    id: String,
    comment: String,
    properties: Vec<String>,
    same_name: Vec<String>,
}   

struct SchemaItemProperty {
    label: String,
    id: String,
    comment: String,
    sub_properties: Vec<String>,
    same_name: Vec<String>,
    domain: Vec<String>
}
struct ToWrite {
    types: HashMap<String, SchemaItemType>,
    properties: HashMap<String, SchemaItemProperty>
}

impl ToWrite {
    pub(crate) fn add_type(&mut self, raw_csv: SchemaItemTypesCsv) {
        let label = raw_csv.label;
        let id = raw_csv.id.clone();
        let comment = raw_csv.comment;
        let same_name: Vec<String> = raw_csv.supersedes
                                     .map(|s| s.split(',').map(|s| s.to_owned()).collect())
                                     .unwrap_or_else(Vec::new);

        let properties: Vec<String> = raw_csv.properties
                                      .split(',')
                                      .map(|s| s.to_owned())
                                      .collect();

        let type_ = SchemaItemType {
            label,
            id: raw_csv.id,
            comment,
            same_name,
            properties,
        };

        self.types.insert(id, type_);
    }

    pub(crate) fn add_property(&mut self, raw_csv: SchemaItemPropertieCsv) {
        let label = raw_csv.label;
        let id = raw_csv.id.clone();
        let comment = raw_csv.comment;
        let same_name: Vec<String> = raw_csv.supersedes
                                     .map(|s: String| s.split(',').map(|s| s.to_owned()).collect())
                                     .unwrap_or_else(Vec::new);
        let domain: Vec<String> = raw_csv.domainIncludes
                                    .split(',')
                                    .map(|s| s.to_owned())
                                    .collect();
        let sub_properties: Vec<String> = raw_csv.subproperties
                                            .map(|s: String| s.split(',').map(|s| s.to_owned()).collect())
                                            .unwrap_or_else(Vec::new);

        let property = SchemaItemProperty {
            label,
            id: raw_csv.id,
            comment,
            same_name,
            domain,
            sub_properties,
        };

        self.properties.insert(id, property);

    }

    pub(crate) fn new() -> Self {
        Self { types: HashMap::new(), properties: HashMap::new() }
    }
}


pub(crate) fn read_csv_schema() {
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
    
}