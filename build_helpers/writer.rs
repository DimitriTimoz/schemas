use std::collections::{HashMap, HashSet};

use convert_case::{Casing, Case};

use super::parse_file::{SchemaItemType, SchemaItemProperty, SchemaItemTypesCsv, SchemaItemPropertieCsv};


pub struct ToWrite {
    types: HashMap<String, SchemaItemType>,
    properties: HashMap<String, SchemaItemProperty>,
    is_domain: HashSet<String>,
    is_defined: HashSet<String>,
}

const TO_REPLACE: [[&str; 2]; 5] = [["Result", "ResultType"], ["Abstract", "AbstractType"], ["Box", "BoxType"], ["Yield", "YieldType"], ["Option", "OptionType"]];
const PRIMITIVE_TYPES: [&str; 7] = ["text", "number", "boolean", "date", "datetime", "url", "time"];
const DIGITS: [&str; 10] = ["Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];

fn id_to_token(id: &str) -> String {
    let id_token = id.split('/').last().unwrap_or(id);
    let id_token = id_token.replace(' ', "");
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
    
    let token: String = token_vec.into_iter().collect();
    // Replace first char if it's a digit
    if !token.is_empty() && token.chars().next().expect("First digit to String shouldn't happen.").is_ascii_digit() {
        let digit = token.chars().next().expect("First digit to String shouldn't happen.").to_digit(10).unwrap() as usize;
        return DIGITS[digit].to_string() + &token[1..];
    }
    token
}

impl ToWrite {
    pub(crate) fn add_type(&mut self, raw_csv: SchemaItemTypesCsv) {
        let label = raw_csv.label;
        let id = raw_csv.id.clone();
        let comment = raw_csv.comment;
        let mut same_name: Vec<String> = raw_csv.supersedes
                                     .map(|s| s.split(',').map(|s| s.to_owned()).collect())
                                     .unwrap_or_else(Vec::new);
        let eq_class: Vec<String> = raw_csv.equivalentClass
                                        .map(|s| s.split(',').map(|s| s.to_owned()).collect())
                                        .unwrap_or_else(Vec::new);
        same_name.extend(eq_class);
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
               is_domain: HashSet::new()
               , is_defined: HashSet::new() }
    }

    pub(crate) fn write_properties_structs(&mut self) -> String {
        let mut output = String::new();
        for (id, property) in &self.properties {
            let id = id.trim();
            let arg_name = &property.label;

            if self.is_defined.contains(id) {
                continue;
            }

            if property.domain.is_empty() {
                continue;
            }

            let token_property = id_to_token(arg_name);
            
            // Enum name as follow: PropertyNameDomain
            let mut prop_type = "#[derive(Debug, Clone)]\n".to_string();
            if property.domain.len() > 1 {
                self.is_domain.insert(id.to_string());

                let enum_token = format!("{}DomainProperty", token_property);
                prop_type.push_str(&format!("pub enum {} {{\n", enum_token));

                for domain in &property.domain {
                    let domain = id_to_token(domain);
                    prop_type.push_str(&format!("\t{}({}),\n", domain, domain));
                }
                prop_type.push_str("}\n\n");    
            } else {
                for domain in &property.domain {
                    prop_type.push_str(&format!("pub struct {}Property({});\n\n",token_property, id_to_token(domain)));
                }
            }

            for same_name in &property.same_name {
                self.is_defined.insert(same_name.to_string());
                let same_name = id_to_token(same_name);
                let is_domain = if self.is_domain.contains(id) { "Domain" } else { "" };
                prop_type.push_str(&format!("pub type {}{}Property = {}{}Property;\n\n", same_name, is_domain, token_property, is_domain));
            }
            self.is_defined.insert(id.to_string());
            output.push_str(&prop_type);

        }

        output
    }

    pub(crate) fn write_type_structs(&mut self) -> String {
        let mut output = String::new();
        for (id, type_) in &self.types {
            let id = id.trim();
            if self.is_defined.contains(id) {
                continue;
            }
            let struct_name = id_to_token(&type_.label);
            if PRIMITIVE_TYPES.contains(&struct_name.to_lowercase().as_str()) {
                continue;
            }
            let mut struct_ = format!("/// {} \n", type_.comment.replace('\n', "\n/// "));
            struct_.push_str("#[derive(Debug, Clone)]\n");
            struct_.push_str(&format!("pub struct {} {{\n", struct_name));
            for prop_type in &type_.properties {
                if prop_type.is_empty() {
                    continue;
                }
                let name: String = id_to_token(prop_type);
                if self.is_domain.contains(prop_type) {
                    struct_.push_str(&format!("\tpub {}: Vec<{}DomainProperty>,\n", name.to_case(Case::Snake), name));
                } else {
                    struct_.push_str(&format!("\tpub {}: Vec<{}Property>,\n", name.to_case(Case::Snake), name));
                }
            }
            struct_.push_str("}\n\n");

            for same_name in &type_.same_name {
                self.is_defined.insert(same_name.to_string());
                let same_name = id_to_token(same_name);
                struct_.push_str(&format!("pub type {} = {};\n\n", same_name, struct_name));
                
            }
            self.is_defined.insert(id.to_string());
            output.push_str(&struct_);

        }
        output
    }
}
