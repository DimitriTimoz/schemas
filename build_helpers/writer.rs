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
    assert!(!patterns.iter().any(|pattern| patterns.iter().any(|other| other != pattern && other.contains(pattern))), "Patterns must not contain each other.");

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

        if new_lines.is_empty() {
            text.replace_range(line.start..line.end+1, "");
        } else {
            text.replace_range(line.clone(), new_lines.join("\n").as_str());
        }
    }
    
    text
}

impl ToWrite {
    pub(crate) fn write_files(table: &mut Table) -> (String, String) {
        let mut to_derive = Vec::new();
        if cfg!(feature = "serde") {
            to_derive.push(String::from("Serialize"));
            to_derive.push(String::from("Deserialize"));
        }

        // properties.rs
        let pattern = include_str!("patterns/prop.rs");
        let mut prop_outputs = Vec::new();
        for property in table.properties.values() {
            let mut output = pattern.to_string();
            output = output.replace("PatternType", &id_to_token(&property.label));
            output = output.replace("PatternDoc", &property.comment.replace('\n', "\n/// "));
            output = output.replace("PatternDerive", &to_derive.join(", "));
            output = multi_replace(output, &["PatternVariant"], vec![property.range_includes.iter().map(|range| id_to_token(&range.id)).collect()]);
            prop_outputs.push(output);
        }

        // types.rs
        let mut types_variants = Vec::new();
        let mut outputs: Vec<String> = Vec::new();
        let pattern = include_str!("patterns/type.rs");
        let prop_type_matcher_pattern = r#"match value {
                Types::PatternPropVariant(value) => self.pattern_property.push(PatternPropertyProp::PatternPropVariant(value)),
                _ => return Err(Error::InvalidType),
            }"#;
        for class in table.classes.values() {
            if PRIMITIVE_LC_TYPES.contains(&class.label.to_lowercase().as_str()) {
                continue;
            }

            let props = class.properties
                .iter()
                .filter_map(|prop| {
                    let tmp = table.properties.get(&prop.id);
                    if tmp.is_none() {
                        println!("Property {} not found.", prop.id);
                    }
                    tmp
                })
                .collect::<Vec<_>>();
            let parents = class.sub_classes
                .iter()
                .filter_map(|sub_class| {
                    let tmp = table.classes.get(&sub_class.id);
                    if tmp.is_none() {
                        println!("Sub class {} not found.", sub_class.id);
                    }
                    tmp
                })
                .collect::<Vec<_>>();

            let mut output = pattern.to_string();
            output = output.replace("PatternType", &id_to_token(&class.label));
            output = output.replace("PatternDoc", &class.comment.replace('\n', "\n/// "));
            output = output.replace("PatternDerive", &to_derive.join(", "));
            output = multi_replace(
                output,
                &["pattern_prop_name_lc", "pattern_property", "PatternProperty", "pattern_prop_type_matcher"],
                vec![
                    props.iter().map(|prop| prop.label.to_lowercase()).collect(),
                    props.iter().map(|prop| id_to_token(&prop.label).to_case(Case::Snake)).collect(),
                    props.iter().map(|prop| id_to_token(&prop.label)).collect(),
                    props.iter()
                        .map(|prop| {
                            let variants = prop.range_includes.iter()
                                .map(|range| id_to_token(&range.id))
                                .collect::<Vec<String>>();
                            let mut matcher = prop_type_matcher_pattern.to_string();
                            matcher = matcher.replace("PatternProperty", &id_to_token(&prop.label));
                            matcher = matcher.replace("pattern_property", &id_to_token(&prop.label).to_case(Case::Snake));
                            matcher = multi_replace(matcher, &["PatternPropVariant"], vec![variants]);
                            matcher
                        })
                        .collect(),
                ]
            );
            output = multi_replace(
                output,
                &["pattern_parent", "PatternParent"],
                vec![
                    parents.iter()
                        .map(|sub_class| id_to_token(&sub_class.label).to_case(Case::Snake))
                        .collect(),
                    parents.iter()
                        .map(|sub_class| id_to_token(&sub_class.label))
                        .collect(),
                ]
            );
            
            outputs.push(output);

        }
        let mut types_code = outputs.join("\n\n");

        // Enum of all types
        for primitive_type in PRIMITIVE_TYPES {
            let primitive_type = id_to_token(primitive_type);
            types_variants.push(primitive_type.clone());
        }
        for ty in table.classes.values() {
            if PRIMITIVE_LC_TYPES.contains(&ty.label.to_lowercase().as_str()) {
                continue;
            }
            types_variants.push(id_to_token(&ty.label));
        }
        let mut code_types = include_str!("patterns/types.rs").to_string();
        code_types = multi_replace(code_types, &["PatternVariant", "pattern_prop_name_lc"], vec![types_variants.clone(), types_variants.iter().map(|v| v.to_lowercase()).collect()]);
        types_code += code_types.as_str();

        // Features
        let mut features = Vec::new();
        for ty in table.classes.values() {
            if PRIMITIVE_LC_TYPES.contains(&ty.label.to_lowercase().as_str()) {
                continue;
            }
            let mut feature = String::from("pattern_name = [\n    \"pattern_prop_dependency_prop\",\n    \"pattern_dependency\",\n]");
            feature = feature.replace("pattern_name", &ty.label.to_lowercase());
            feature = multi_replace(feature, &["pattern_dependency"], vec![ty.sub_classes.iter().filter_map(|sub_class| table.classes.get(&sub_class.id)).map(|c| c.label.to_lowercase()).collect::<Vec<_>>()]);
            feature = multi_replace(feature, &["pattern_prop_dependency"], vec![ty.properties.iter().filter_map(|prop| table.properties.get(&prop.id)).map(|p| p.label.to_lowercase()).collect::<Vec<_>>()]);
            features.push(feature);
        }
        for prop in table.properties.values() {
            let mut feature = String::from("pattern_name_prop = [\n    \"pattern_dependency\",\n]");
            feature = feature.replace("pattern_name", &prop.label.to_lowercase());
            feature = multi_replace(feature, &["pattern_dependency"], vec![prop.range_includes.iter().filter_map(|range| table.classes.get(&range.id)).map(|c| c.label.to_lowercase()).collect::<Vec<_>>()]);
            features.push(feature);
        }
        std::fs::write("src/features", features.join("\n\n")).expect("Unable to write file");

        // Debugging
        // std::fs::write("src/test.rs", types_code.clone()).expect("Unable to write file");
        // std::fs::write("src/test2.rs", prop_outputs.join("\n\n\n")).expect("Unable to write file");

        (prop_outputs.join("\n\n"), types_code)
    }
}
