use convert_case::{Case, Casing};

use super::parse_file::Table;

pub struct ToWrite {}

const TO_REPLACE: [[&str; 2]; 6] = [
    ["Result", "ResultType"],
    ["Abstract", "AbstractType"],
    ["Box", "BoxType"],
    ["Yield", "YieldType"],
    ["Option", "OptionType"],
    ["PriceRange", "PriceRangeType"],
];
const PRIMITIVE_TYPES: [&str; 7] = [
    "text", "number", "boolean", "date", "datetime", "url", "time",
];
const DIGITS: [&str; 10] = [
    "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];

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
            return to_replace[1].to_string();
        }
    }

    let token: String = token_vec.into_iter().collect();
    // Replace first char if it's a digit
    if !token.is_empty()
        && token
            .chars()
            .next()
            .expect("First digit to String shouldn't happen.")
            .is_ascii_digit()
    {
        let digit = token
            .chars()
            .next()
            .expect("First digit to String shouldn't happen.")
            .to_digit(10)
            .unwrap() as usize;
        return DIGITS[digit].to_string() + &token[1..];
    }
    token
}

impl ToWrite {

    fn header_with_doc(doc: &str) -> String {
        format!(
            "/// {}\n#[derive(Debug, Clone)]\n",
            doc.replace('\n', "\n/// ")
        )
    }

    pub(crate) fn write_files(table: &mut Table) -> (String, String) {
        let mut props_output = String::new();
        let mut classes_output = String::new();

        // properties.rs
        for property in table.properties.values() {
            let mut prop_output = Self::header_with_doc(&property.comment);

            match property.range_includes.len() {
                0 => {
                    println!("Property {} has no range inclDudes.", property.id);
                    prop_output += &format!("pub struct {}Prop;\n", id_to_token(&property.label));

                    // Impl of new()
                    prop_output += &format!(
                        "\nimpl {}Prop {{\n    pub fn new() -> Self {{\n        Self\n    }}\n}}\n\n",
                        id_to_token(&property.label)
                    );

                }
                1 => {
                    // Struct
                    let range = property
                        .range_includes
                        .first()
                        .expect("Range includes should have at least one element.");
                    let args = if !property.sub_properties.is_empty() {
                        let mut args = String::from("{\n");
                        let mut sub_props_names: Vec<String> = Vec::new();
                        for sub_prop in &property.sub_properties {
                            let sub_prop =
                                if let Some(sub_prop) = table.properties.get(&sub_prop.id) {
                                    sub_prop
                                } else {
                                    println!("Sub property {} not found.", sub_prop.id);
                                    continue;
                                };
                            let range = if sub_prop.range_includes.len() > 1 {
                                "Range"
                            } else {
                                ""
                            };
                            let sub_prop_name = id_to_token(&sub_prop.label).to_case(Case::Snake);
                            sub_props_names.push(sub_prop_name.clone());
                            args += &format!(
                                "  pub {}: Vec<{}{}Prop>,\n",
                                sub_prop_name,
                                id_to_token(&sub_prop.label),
                                range
                            );
                        }
                        args += "}\n";

                        // Impl of new()
                        let sub_props_names = sub_props_names.iter().map(|s| s.to_string() + ": Vec::new()").collect::<Vec<String>>();
                        let sub_props_names = sub_props_names.join(",\n  ");
                        args += &format!(
                            r#"
impl {}Prop {{
    pub fn new() -> Self {{
        Self {{
            {}
        }}    
    }}
}}"#,
                            id_to_token(&property.label),
                            sub_props_names,
                        );
                        args
                    } else {
                        format!("(pub {});\n", id_to_token(&range.id))
                    };

                    prop_output +=
                        &format!("pub struct {}Prop {}\n", id_to_token(&property.label), args);
                }
                _ => {
                    // Enum
                    let label = &property.label;
                    prop_output += &format!("pub enum {}RangeProp {{\n", id_to_token(label));
                    for range in &property.range_includes {
                        prop_output += &format!(
                            "    {}(Vec<{}>),\n",
                            id_to_token(&range.id),
                            id_to_token(&range.id)
                        );
                    }
                    prop_output += "}\n\n";
                }
            }
            props_output += &prop_output;
        }

        // types.rs
        let mut types_variations = String::new();
        for class in table.classes.values() {
            if PRIMITIVE_TYPES.contains(&class.label.to_lowercase().as_str()) {
                continue;
            }
            let mut class_outuput = Self::header_with_doc(&class.comment);
            class_outuput += &format!("pub struct {} {{\n", id_to_token(&class.label));
            for prop in &class.properties {
                let prop = if let Some(prop) = table.properties.get(&prop.id) {
                    prop
                } else {
                    println!("Property {} not found.", prop.id);
                    continue;
                };

                let range_suffix = if prop.range_includes.len() > 1 {
                    "Range"
                } else {
                    ""
                };
                let prop_type = id_to_token(&prop.label);
                class_outuput += &format!(
                    "    pub {}: Vec<{}{}Prop>,\n",
                    prop_type.to_case(Case::Snake),
                    prop_type,
                    range_suffix
                );
            }
            match class.sub_classes.len() {
                1 => {
                    let sub_class =
                        if let Some(sub_class) = table.classes.get(&class.sub_classes[0].id) {
                            sub_class
                        } else {
                            println!("Sub class {} not found.", class.sub_classes[0].id);
                            continue;
                        };
                    class_outuput +=
                        &format!("    pub sub_class: {},\n", id_to_token(&sub_class.label));
                }
                0 => {}
                _ => {
                    class_outuput += &format!(
                        "    pub sub_classes: [{}SubClasses; {}],\n",
                        id_to_token(&class.label),
                        class.sub_classes.len()
                    );
                }
            }
            class_outuput += "}\n\n";

            // Heritance enum
            if class.sub_classes.len() > 1 {
                let mut herticance_enum = format!(
                    "#[derive(Debug, Clone)]\npub enum {}SubClasses {{\n",
                    id_to_token(&class.label)
                );

                for sub_class in &class.sub_classes {
                    let sub_class = if let Some(sub_class) = table.classes.get(&sub_class.id) {
                        sub_class
                    } else {
                        println!("Sub class {} not found.", sub_class.id);
                        continue;
                    };
                    let sub_class = id_to_token(&sub_class.label);
                    herticance_enum += &format!(
                        "    {}({}),\n",
                        sub_class,
                        sub_class
                    );
                }
                herticance_enum += "}\n\n";
                class_outuput += &herticance_enum;
            }
            let class = id_to_token(&class.label);
            types_variations += &format!(
                "   {}({}),\n",
                &class,
                &class
            );
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

        classes_output += &format!("pub enum Types {{\n{}\n}}\n\n", types_variations);

        (props_output, classes_output, )
    }
}
