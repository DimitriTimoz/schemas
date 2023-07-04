use convert_case::{Casing, Case};

use super::parse_file::{Table};


pub struct ToWrite {
}

const TO_REPLACE: [[&str; 2]; 6] = [["Result", "ResultType"], ["Abstract", "AbstractType"], ["Box", "BoxType"], ["Yield", "YieldType"], ["Option", "OptionType"], ["PriceRange", "PriceRangeType"]];
const PRIMITIVE_TYPES: [&str; 7] = ["text", "number", "boolean", "date", "datetime", "url", "time"];
const DIGITS: [&str; 10] = ["Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];

fn id_to_token(id: &str) -> String {
    let id_token = id.trim_start_matches("schema:");
    let id_token = id_token.split('/').last().unwrap_or(id);
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

    pub(crate) fn write_files(table: &mut Table) -> (String, String) {
        let mut props_output = String::new();
        let mut classes_output = String::new();

        // properties.rs
        for property in table.properties.values() {
            let mut prop_output = format!("/// {}\n#[derive(Debug, Clone)]\n", property.comment.replace('\n', "\n/// "));

            match property.range_includes.len() {
                0 => {
                    println!("Property {} has no range inclDudes.", property.id);
                    prop_output += &format!("pub struct {}Prop;\n", id_to_token(&property.label));
                },
                1 => {
                    // Struct
                    let range = property.range_includes.first().expect("Range includes should have at least one element.");
                    let args = if !property.sub_properties.is_empty() {
                        let mut args = String::from("{\n");
                        for sub_prop in &property.sub_properties {
                            let sub_prop = if let Some(sub_prop) = table.properties.get(&sub_prop.id) {
                                sub_prop
                            } else {
                                println!("Sub property {} not found.", sub_prop.id);
                                continue;
                            };
                            args += &format!("            pub {}: Vec<{}Prop>,\n", id_to_token(&sub_prop.label).to_case(Case::Snake), id_to_token(&sub_prop.label));
                        }
                        args += "        }\n";
                        args
                    } else {
                        format!("({});\n", id_to_token(&range.id))
                    };
                 
                    prop_output += &format!("pub struct {}Prop {}\n", id_to_token(&property.label), args);

                },
                _ => {
                    // Enum
                    let label = &property.label;
                    prop_output += &format!("pub enum {}RangeProp {{\n", id_to_token(label));
                    for range in &property.range_includes {
                        prop_output += &format!("    {}(Vec<{}>),\n", id_to_token(&range.id), id_to_token(&range.id));
                    }
                    prop_output += "}\n\n";
    
                }
            }
            props_output += &prop_output;
        }

        // types.rs
        for class in table.classes.values() {
            if PRIMITIVE_TYPES.contains(&class.label.to_lowercase().as_str()) {
                continue;
            }
            let mut class_outuput = format!("/// {}\n#[derive(Debug, Clone)]\n", class.comment.replace('\n', "\n/// "));
            class_outuput += &format!("pub struct {} {{\n", id_to_token(&class.label));
            for prop in &class.properties {
                let prop = if let Some(prop) = table.properties.get(&prop.id) {
                    prop
                } else {
                    println!("Property {} not found.", prop.id);
                    continue;
                };
                let range_suffix = if table.is_domain.contains(prop.id.as_str()) {
                    "Range"
                } else {
                    ""
                };
                let prop_type = id_to_token(&prop.label);
                class_outuput += &format!("    pub {}: Vec<{}{}Prop>,\n", id_to_token(&prop.label).to_case(Case::Snake), prop_type, range_suffix);
            }
            if !class.sub_classes.is_empty() {
                class_outuput += &format!("    pub sub_classes: Vec<{}SubClasses>,\n", id_to_token(&class.label));
            }             
            class_outuput += "}\n\n";

            // Heritance enum
            if !class.sub_classes.is_empty() {
                let mut herticance_enum = format!("#[derive(Debug, Clone)]\npub enum {}SubClasses {{\n", id_to_token(&class.label));

                for sub_class in &class.sub_classes {
                    let sub_class = if let Some(sub_class) = table.classes.get(&sub_class.id) {
                        sub_class
                    } else {
                        println!("Sub class {} not found.", sub_class.id);
                        continue;
                    };
                    herticance_enum += &format!("    {}({}),\n", id_to_token(&sub_class.label), id_to_token(&sub_class.label));
                }
                herticance_enum += "}\n\n";
                class_outuput += &herticance_enum;
            }
            classes_output += &class_outuput;
        }

        for (label, id) in &table.same_name {
            let class = if let Some(class) = table.classes.get(id) {
                class
            } else {
                println!("Class {} not found.", id);
                continue;
            };

            classes_output += &format!("pub type {} = {};\n\n", label, &class.label);

        }
        (props_output, classes_output)
    }

}
