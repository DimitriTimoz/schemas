use std::{collections::{HashMap, HashSet}, fmt::format};

use convert_case::{Casing, Case};

use super::parse_file::{SchemaItemType, SchemaItemProperty, SchemaItemTypesCsv, SchemaItemPropertieCsv};



pub struct ToWrite {
    types: HashMap<String, SchemaItemType>,
    properties: HashMap<String, SchemaItemProperty>,
    is_domain: HashSet<String>,
}

fn id_to_capitalized(id: &str) -> String {
    let id_token = id.split('/').last().unwrap_or(id);

    let mut token_vec: Vec<char> = id_token.chars().collect();
    token_vec[0] = token_vec[0].to_uppercase().next().unwrap();
    // To String
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
        Self { types: HashMap::new(), properties: HashMap::new(),
               is_domain: HashSet::new() }
    }

    pub(crate) fn write_properties_structs(&mut self) -> String {
        let mut output = String::new();
        for (id, property) in &self.properties {
            let arg_name = &property.label;

            // Write the domain enum:
            let mut domain_variations = Vec::new();
            
            if property.domain.len() <= 1 {
                
            }

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
            let mut token_vec: Vec<char> = arg_name.chars().collect();
            token_vec[0] = token_vec[0].to_uppercase().next().unwrap();
            // To String
            let enum_token: String = token_vec.into_iter().collect();

            let enum_token = format!("{}Domain", enum_token);

            let mut domain_enum = format!("pub enum {} {{\n", enum_token);
            for domain in &domain_variations {
                domain_enum.push_str(&format!("\t{}({}),\n", domain, domain));
            }
            domain_enum.push_str("}\n");

            // The property struct
            let mut property_struct = format!("/// {} \n", property.comment.replace('\n', "\n/// "));
            property_struct.push_str("#[derive(Debug, Clone)]\n");
            property_struct.push_str(&format!("pub struct {} {{\n", id_to_capitalized(arg_name)));
            for sub_property in &property.sub_properties {
                println!("sub_property: {}", sub_property);
                let name = id_to_capitalized(sub_property);
                property_struct.push_str(&format!("\tpub {}: Vec<{}>,\n", name.to_case(Case::Snake), id_to_capitalized(sub_property)));
            }
            property_struct.push_str("}\n\n");

            output.push_str(&property_struct);

            output.push_str(&domain_enum);
        }

        output
    }
}
