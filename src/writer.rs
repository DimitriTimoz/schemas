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
pub const PRIMITIVE_TYPES: [&str; 10] = [
    "Text",
    "Number",
    "Integer",
    "Boolean",
    "Date",
    "DateTime",
    "Url",
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

const PRIMITIVE_TYPE_INNER: [&str; 10] = [
    "String",
    "f64",
    "i64",
    "bool",
    "String",
    "String",
    "String",
    "String",
    "String",
    "String",
];

const DIGITS: [&str; 10] = [
    "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];

pub fn id_to_token(id: &str) -> String {
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

    let mut token: String = token_vec.into_iter().collect();
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

pub fn id_to_inner(id: &str) -> String {
    let token = id_to_token(id);

    let primitive_idx = PRIMITIVE_LC_TYPES
        .iter()
        .position(|primitive| primitive == &token.to_lowercase());
    if let Some(idx) = primitive_idx {
        PRIMITIVE_TYPE_INNER[idx].to_string()
    } else {
        token
    }
}

#[track_caller]
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
    pub(crate) fn write_files(table: &mut Table) -> (String, String, String, String) {
        // properties.rs
        let pattern = include_str!("patterns/prop.rs");
        let mut prop_outputs = Vec::new();
        for property in table.properties.values() {
            let mut output = pattern.to_string();
            output = output.replace("PatternType", &id_to_token(&property.label));
            output = output.replace("PatternDoc", &property.doc());
            output = output.replace("pattern_feature", &property.feature_name());
            output = multi_replace(output, &["PatternVariant", "PatternInnerVariant"], vec![property.range_includes.iter().map(|range| id_to_token(&range.id)).collect(), property.range_includes.iter().map(|range| id_to_inner(&range.id)).collect()]);
            output = multi_replace(output, &["PatternPrimitiveVariant"], vec![property.range_includes.iter().map(|range| id_to_token(&range.id)).filter(|t| PRIMITIVE_LC_TYPES.contains(&t.to_lowercase().as_str())).collect()]);
            output = multi_replace(output, &["PatternObjectVariant"], vec![property.range_includes.iter().map(|range| id_to_token(&range.id)).filter(|t| !PRIMITIVE_LC_TYPES.contains(&t.to_lowercase().as_str())).collect()]);
            prop_outputs.push(output);
        }

        // types.rs
        let mut outputs: Vec<String> = Vec::new();
        let pattern = include_str!("patterns/class.rs");
        for (class_id, class) in table.classes.iter() {
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
        
            let mut parents = vec![class];
            loop {
                let start_len = parents.len();
                for parent in parents.clone() {
                    parents.extend(parent.sub_classes.iter().filter_map(|sub_class| {
                        let tmp = table.classes.get(&sub_class.id);
                        if tmp.is_none() {
                            println!("Sub class {} not found.", sub_class.id);
                        }
                        tmp
                    }));
                }
                parents.sort_by_key(|class| &class.label);
                parents.dedup_by_key(|class| &class.label);
                if parents.len() == start_len {
                    break;
                }
            }

            let mut children = vec![class_id.to_owned()];
            loop {
                let start_len = children.len();
                for child in children.clone() {
                    children.extend(table.classes.iter().filter(|(_,c)| c.sub_classes.iter().any(|sub_class| sub_class.id == child)).map(|(id,_)| id.to_owned()));
                }
                children.sort();
                children.dedup();
                if children.len() == start_len {
                    break;
                }
            }
            children = children.into_iter().filter_map(|child| table.classes.get(&child)).map(|child| child.label.to_owned()).collect::<Vec<_>>();
            
            let mut output = pattern.to_string();
            output = output.replace("PatternType", &id_to_token(&class.label));
            output = output.replace("PatternDoc", &class.doc());
            output = output.replace("pattern_feature", &class.feature_name());
            output = multi_replace(output, &["pattern_child_ty_lc"], vec![children.iter().map(|child| child.to_lowercase()).collect()]);
            output = multi_replace(
                output,
                &["pattern_prop_ty_lc", "pattern_property", "PatternProperty", "pattern_prop_feature"],
                vec![
                    props.iter().map(|prop| prop.label.to_lowercase()).collect(),
                    props.iter().map(|prop| id_to_token(&prop.label).to_case(Case::Snake)).collect(),
                    props.iter().map(|prop| id_to_token(&prop.label)).collect(),
                    props.iter().map(|prop| prop.feature_name()).collect(),
                ]
            );
            output = multi_replace(output, &["PatternParent"], vec![parents.iter().map(|sub_class| id_to_token(&sub_class.label)).collect()]);
            
            outputs.push(output);

        }

        // Enum of all types
        let mut types_variants = Vec::new();
        for ty in table.classes.values() {
            if PRIMITIVE_LC_TYPES.contains(&ty.label.to_lowercase().as_str()) {
                continue;
            }
            types_variants.push((id_to_token(&ty.label), ty.feature_name()));
        }
        let mut code_types = include_str!("patterns/types.rs").to_string();
        code_types = multi_replace(
            code_types,
            &["pattern_variant_feature", "PatternVariant", "pattern_prop_ty_lc"],
            vec![
                types_variants.iter().map(|(_,f)| f).cloned().collect(),
                types_variants.iter().map(|(v,_)| v).cloned().collect(),
                types_variants.iter().map(|(v,_)| v.to_lowercase()).collect()
            ]
        );

        // Features
        let mut features = Vec::new();
        for ty in table.classes.values() {
            if PRIMITIVE_LC_TYPES.contains(&ty.label.to_lowercase().as_str()) {
                continue;
            }
            let mut feature = String::from("pattern_name = []");
            feature = feature.replace("pattern_name", &ty.feature_name());
            features.push(feature);
        }
        for prop in table.properties.values() {
            let mut feature = String::from("pattern_name = [\n    \"pattern_dependency\",\n]");
            feature = feature.replace("pattern_name", &prop.feature_name());
            feature = multi_replace(feature, &["pattern_dependency"], vec![
                prop.range_includes.iter().filter_map(|range| table.classes.get(&range.id)).filter(|p| !PRIMITIVE_LC_TYPES.contains(&p.label.to_lowercase().as_str())).map(|c| c.feature_name()).collect::<Vec<_>>()
            ]);
            features.push(feature);
        }

        (prop_outputs.join("\n\n"), outputs.join("\n\n"), features.join("\n"), code_types)
    }
}
