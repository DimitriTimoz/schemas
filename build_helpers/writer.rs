use convert_case::{Case, Casing};

use super::parse_file::{Id, Table};

pub struct ToWrite {}

const TO_REPLACE: [[&str; 2]; 6] = [
    ["Result", "ResultType"],
    ["Abstract", "AbstractType"],
    ["Box", "BoxType"],
    ["Yield", "YieldType"],
    ["Option", "OptionType"],
    ["PriceRange", "PriceRangeType"],
];
const PRIMITIVE_TYPES: [&str; 10] = [
    "Text",
    "Number",
    "Integer",
    "Boolean",
    "Date",
    "DateTime",
    "URL",
    "Time",
    "XPathType",
    "CssSelectorType",
];

const PRIMITIVE_LC_TYPES: [&str; 10] = [
    "text",
    "number",
    "integer",
    "boolean",
    "date",
    "datetime",
    "url",
    "time",
    "xpathtype",
    "cssselectortype",
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

fn multi_replace(mut text: String, patterns: &'static [&'static str], values: Vec<Vec<String>>) -> String {
    assert!(!patterns.is_empty(), "Patterns and values must not be empty.");
    assert!(patterns.len() == values.len(), "Patterns and values must have the same length.");
    assert!(values.iter().all(|v| v.len() == values[0].len()), "Values must equal lenghts.");

    let mut line_ranges = Vec::new();
    for line in text.lines() {
        let begin = line.as_ptr() as usize - text.as_ptr() as usize;
        let end = begin + line.len();
        line_ranges.push(begin..end);
    }

    for line in line_ranges.into_iter().rev() {
        let mut is_to_be_replaced = false;
        for pattern in patterns {
            if text[line.clone()].contains(pattern) {
                is_to_be_replaced = true;
                break;
            }
        }
        if !is_to_be_replaced {
            continue;
        }

        let mut new_lines: Vec<String> = Vec::new();
        for i in 0..values[0].len() {
            let mut new_line = text[line.clone()].to_string();
            for (pattern, values) in patterns.iter().zip(&values) {
                new_line = new_line.replace(pattern, &values[i]);
            }
            new_lines.push(new_line);
        }

        text.replace_range(line.clone(), new_lines.join("\n").as_str());
    }
    
    text
}

impl ToWrite {
    fn header_with_doc(doc: &str) -> String {
        let mut to_derive = vec!["Debug", "Clone"];
        if cfg!(feature = "serde") {
            to_derive.push("Serialize");
            to_derive.push("Deserialize");
        }
        format!(
            "/// {}\n#[derive({})]\n",
            doc.replace('\n', "\n/// "),
            to_derive.join(", ")
        )
    }

    pub(crate) fn write_files(table: &mut Table) -> (String, String) {
        let mut props_output = String::new();
        let mut classes_output = String::new();

        // properties.rs
        for property in table.properties.values() {
            let mut prop_output = Self::header_with_doc(&property.comment);
            let mut range_include = property.range_includes.clone();
            let txt = Id {
                id: "schema:Text".to_string(),
            };
            if !range_include.contains(&txt) {
                range_include.insert(txt);
            }
            match range_include.len() {
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
                    let range = range_include
                        .clone()
                        .drain()
                        .next()
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

                            let mut range_include = sub_prop.range_includes.clone();
                            let txt = Id {
                                id: "schema:Text".to_string(),
                            };
                            if !range_include.contains(&txt) {
                                range_include.insert(txt);
                            }
                            let sub_prop_name = id_to_token(&sub_prop.label).to_case(Case::Snake);
                            sub_props_names.push(sub_prop_name.clone());
                            args += &format!(
                                "  pub {}: Vec<{}Prop>,\n",
                                sub_prop_name,
                                id_to_token(&sub_prop.label),
                            );
                        }
                        args += "}\n";

                        // Impl of new()
                        let sub_props_names = sub_props_names
                            .iter()
                            .map(|s| s.to_string() + ": Vec::new()")
                            .collect::<Vec<String>>();
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
                    prop_output += &format!("pub enum {}Prop {{\n", id_to_token(label));
                    for range in &range_include {
                        prop_output += &format!(
                            "    {}({}),\n",
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
        let mut outputs: Vec<String> = Vec::new();
        let pattern = include_str!("type-pattern.rs");
        for class in table.classes.values() {
            let mut output = pattern.to_string();
            output = output.replace("PatternType", &id_to_token(&class.label));
            output = output.replace("PatternDoc", &class.comment.replace('\n', "\n/// "));
            output = multi_replace(output, &["multi_pattern_property", "MultiPatternProperty"], vec![vec![String::from("wip"), String::from("wop"), String::from("wup")], vec![String::from("zap"), String::from("zip"), String::from("zoup")]]);

            if PRIMITIVE_LC_TYPES.contains(&class.label.to_lowercase().as_str()) {
                continue;
            }
            let mut class_outuput = Self::header_with_doc(&class.comment);
            let class_name = id_to_token(&class.label);
            class_outuput += &format!("pub struct {} {{\n", class_name);
            let mut props_init = String::new();
            for prop in &class.properties {
                let prop = if let Some(prop) = table.properties.get(&prop.id) {
                    prop
                } else {
                    println!("Property {} not found.", prop.id);
                    continue;
                };
                let mut range_include = prop.range_includes.clone();
                let txt = Id {
                    id: "schema:Text".to_string(),
                };
                if !range_include.contains(&txt) {
                    range_include.insert(txt);
                }
                let prop_type = id_to_token(&prop.label);
                let arg_name = prop_type.to_case(Case::Snake);
                props_init += &format!("        {}: Vec::new(),\n", arg_name);
                class_outuput += &format!("    pub {}: Vec<{}Prop>,\n", arg_name, prop_type);
            }
            let mut sub_classes = Vec::new();
            match class.sub_classes.len() {
                1 => {
                    let sub_class_id = class.sub_classes.clone().drain().next().unwrap().id;
                    let sub_class = if let Some(sub_class) = table.classes.get(&sub_class_id) {
                        sub_class
                    } else {
                        println!("Sub class {sub_class_id} not found.");
                        continue;
                    };
                    let token = id_to_token(&sub_class.label);
                    sub_classes.push(token.clone());
                    class_outuput += &format!("    pub sub_class: {},\n", token);
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
                let mut to_derive = vec!["Debug", "Clone"];
                if cfg!(feature = "serde") {
                    to_derive.push("Serialize");
                    to_derive.push("Deserialize");
                }
                let mut heritance_enum = format!(
                    "#[derive({})]\npub enum {}SubClasses {{\n",
                    to_derive.join(", "),
                    class_name
                );

                let mut patterns = String::new();
                for sub_class in &class.sub_classes {
                    let sub_class = if let Some(sub_class) = table.classes.get(&sub_class.id) {
                        sub_class
                    } else {
                        println!("Sub class {} not found.", sub_class.id);
                        continue;
                    };
                    let sub_class = id_to_token(&sub_class.label);
                    sub_classes.push(sub_class.clone());
                    heritance_enum += &format!("    {}({}),\n", sub_class, sub_class);

                    patterns += &format!(
                        "           {}SubClasses::{}(object) => object.add_item(name, item),\n",
                        class_name, sub_class
                    );
                }
                heritance_enum += "}\n\n";

                heritance_enum += &format!(
                    r#"
impl {class_name}SubClasses {{
    pub fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {{
        match self {{
{patterns}_ => Err(Error::InvalidProperty),
        }}
    }}
}}

                "#
                );
                class_outuput += &heritance_enum;
            }
            types_variations += &format!("   {}({}),\n", &class_name, &class_name);

            // Impl of schema trait

            let (sub_class_init, sub_class_test) = match sub_classes.len() {
                1 => (
                    format!("sub_class: {}::new(),", sub_classes.first().unwrap()),
                    String::from("self.sub_class.add_item(name, item)"),
                ),
                0 => (String::new(), String::from("Err(Error::InvalidProperty)")),
                _ => {
                    let inits = sub_classes
                        .iter()
                        .map(|el| format!("{}SubClasses::{}({}::new())", class_name, el, el))
                        .collect::<Vec<String>>()
                        .join(", ");

                    (
                        format!("sub_classes: [{}],", inits),
                        r#"
                for mut sub_class in self.sub_classes.iter_mut() {
                    todo!("If error continue to next sub class and if success return Ok(()).");
                    sub_class.add_item(name.clone(), item.clone())?;
                }
                Ok(())
                    "#
                        .to_string(),
                    )
                }
            };

            

            let implementation = format!(
                r#"impl Schema for {class_name} {{
    fn new() -> Self {{
        Self {{
            {props_init}
            {sub_class_init}
        }}
    }}           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {{
        match name.to_lowercase().as_str() {{
            "" => {{}},
            _ => return Err(Error::InvalidProperty),
        }}
        Ok(())
    }}

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {{
        match name.to_lowercase().as_str() {{
            "" => match item {{
                Types::Date(date) => {{
                    Ok(())
                }}
                _ => Err(Error::InvalidType),
            }},
            _ => {{
                {sub_class_test}
            }},
        }}
    }}
  
}}
"#
            );

            class_outuput += &implementation;
            classes_output += &class_outuput;

            outputs.push(output);
        }

        std::fs::write("src/test.rs", outputs.join("\n\n\n")).expect("Unable to write file");

        // Enum of all types
        for primitive_type in PRIMITIVE_TYPES {
            let primitive_type = id_to_token(primitive_type);
            types_variations += &format!("   {}({}),\n", primitive_type, primitive_type);
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

        let mut to_derive = vec!["Debug", "Clone"];
        if cfg!(feature = "serde") {
            to_derive.push("Serialize");
            to_derive.push("Deserialize");
        }
        classes_output += &format!(
            "#[derive({})]\npub enum Types {{\n{}\n}}\n\n",
            to_derive.join(", "),
            types_variations
        );

        (props_output, classes_output)
    }
}
