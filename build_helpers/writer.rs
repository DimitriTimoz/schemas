use std::collections::{HashMap, HashSet};

use convert_case::{Casing, Case};

use super::parse_file::{SchemaItemType, SchemaItemProperty, SchemaItemTypesCsv, SchemaItemPropertieCsv};


pub struct ToWrite {
    types: HashMap<String, SchemaItemType>,
    properties: HashMap<String, SchemaItemProperty>,
    is_domain: HashSet<String>,
}

const TO_REPLACE: [[&str; 2]; 4] = [["Result", "ResultType"], ["Abstract", "AbstractType"], ["Box", "BoxType"], ["Yield", "YieldType"]];
const PRIMITIVE_TYPES: [&str; 7] = ["text", "number", "boolean", "date", "datetime", "url", "time"];

fn id_to_capitalized(id: &str) -> String {
    let id_token = id.split('/').last().unwrap_or(id);

    let mut token_vec: Vec<char> = id_token.chars().collect();
    if !token_vec.is_empty() {
        token_vec[0] = token_vec[0].to_uppercase().next().unwrap();
    }
    // To String
    for to_replace in TO_REPLACE {
        if id_token.to_lowercase() == to_replace[0].to_lowercase() {
            return to_replace[1].to_string()
        }
    }
    token_vec.into_iter().collect()
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
        let label = raw_csv.label.clone();
        let id = raw_csv.id.clone();
        let comment = raw_csv.comment;
        let same_name: Vec<String> = raw_csv.supersedes
                                     .map(|s: String| s.split(',').map(|s| s.to_owned()).collect())
                                     .unwrap_or_else(Vec::new);
        let domain: Vec<String> = raw_csv.rangeIncludes
                                    .map(|s: String| s.split(',').map(|s| s.to_owned()).collect())
                                    .unwrap_or_else(Vec::new);
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
        Self { types: HashMap::new(), properties: HashMap::new(),
               is_domain: HashSet::new() }
    }

    pub(crate) fn write_properties_structs(&mut self) -> String {
        let mut output = String::new();
        for (id, property) in &self.properties {
            let arg_name = &property.label;

            // Write the domain enum:
            let mut domain_variations = Vec::new();

            for eq_type in &property.domain {
                // Type id afer the last /
                let id_token = eq_type.split('/').last();
                if let Some(id_token) = id_token {
                    domain_variations.push(id_token);
                } else {
                    eprintln!("Error: id_token is None");
                }
            }   

            // Enum name as follow: PropertyNameDomain
            let enum_token = id_to_capitalized(arg_name);
            
            let enum_token = format!("{}Domain", enum_token);

            let mut domain_enum = format!("pub enum {} {{\n", enum_token);
            for domain in &domain_variations {
                domain_enum.push_str(&format!("\t{}({}),\n", domain, domain));
            }
            domain_enum.push_str("}\n");

            // The property struct
            let mut property_struct = format!("/// {} \n", property.comment.replace('\n', "\n/// "));
            property_struct.push_str("#[derive(Debug, Clone)]\n");
            let struct_name = id_to_capitalized(arg_name);
            property_struct.push_str(&format!("pub struct {} {{\n", struct_name));
            for sub_property in &property.sub_properties {
                let name: String = id_to_capitalized(sub_property);
                if name.is_empty() {
                    continue;
                }
                property_struct.push_str(&format!("\tpub {}: Vec<{}>,\n", name.to_case(Case::Snake), id_to_capitalized(sub_property)));
            }
            property_struct.push_str("}\n\n");

            // Types with the same struct but different name
            println!("{:?}", property.same_name); 
            for name in &property.same_name {
                let mut property_struct = format!("/// {} \n", property.comment.replace('\n', "\n/// "));
                let name: String = id_to_capitalized(name);
                property_struct.push_str(&format!("pub type {} = {};\n", name, struct_name));
                property_struct.push('\n');
            }

            output.push_str(&property_struct);

            output.push_str(&domain_enum);
        }

        output
    }

    pub(crate) fn write_type_structs(&mut self) -> String {
        let mut output = String::new();
        for (id, property) in &self.types {
            let struct_name = id_to_capitalized(&property.label);
            if PRIMITIVE_TYPES.contains(&struct_name.to_lowercase().as_str()) {
                continue;
            }
            let mut struct_ = format!("/// {} \n", property.comment.replace('\n', "\n/// "));
            struct_.push_str("#[derive(Debug, Clone)]\n");
            struct_.push_str(&format!("pub struct {} {{\n", struct_name));
            for property in &property.properties {
                if property.is_empty() {
                    continue;
                }
                let name: String = id_to_capitalized(property);
                if self.is_domain.contains(property) {
                    struct_.push_str(&format!("\tpub {}: Vec<{}Domain>,\n", name.to_case(Case::Snake), name));
                } else {
                    struct_.push_str(&format!("\tpub {}: Vec<{}>,\n", name.to_case(Case::Snake), name));
                }
            }
            struct_.push_str("}\n\n");
            output.push_str(&struct_);

        }
        output
    }
}
