use std::collections::HashSet;

use convert_case::{Casing, Case};

use super::parse_file::{Root, Graph, RangeIncludes, Table};


pub struct ToWrite {
    table: Table,
    is_domain: HashSet<String>,
    is_defined: HashSet<String>,
}

const TO_REPLACE: [[&str; 2]; 5] = [["Result", "ResultType"], ["Abstract", "AbstractType"], ["Box", "BoxType"], ["Yield", "YieldType"], ["Option", "OptionType"]];
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

    pub(crate) fn write_files(table: &Table) -> (String, String) {
        let mut props_output = String::new();
        let mut class_output = String::new();

        // properties.rs
        for property in table.properties.values() {
            let mut prop_output = String::from("#[derive(Debug, Clone)]\n");
            if property.range_includes.len() > 1 {
                // Enum 
                prop_output += &format!("pub enum {}Range {{\n", id_to_token(&property.id));
                for range in &property.range_includes {
                    prop_output += &format!("    {}({}),\n", id_to_token(&range.id), id_to_token(&range.id));
                }
                prop_output += "}\n\n";
            } else if property.range_includes.len() == 1 {
                // Struct
                let range = property.range_includes.first().expect("Range includes should have at least one element.");
                prop_output += &format!("pub struct {}Range({});\n", id_to_token(&property.id), id_to_token(&range.id));
            } else {
                println!("Property {} has no range includes.", property.id);
                continue;
            }
            props_output += &prop_output;
        }

        (props_output, class_output)
    }

}
