use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::writer::{PRIMITIVE_TYPES, id_to_token};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Root {
    #[serde(rename = "@context")]
    pub(crate) context: Context,
    #[serde(rename = "@graph")]
    pub(crate) graph: Vec<Graph>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Context {
    pub(crate) brick: String,
    pub(crate) csvw: String,
    pub(crate) dc: String,
    pub(crate) dcam: String,
    pub(crate) dcat: String,
    pub(crate) dcmitype: String,
    pub(crate) dcterms: String,
    pub(crate) doap: String,
    pub(crate) foaf: String,
    pub(crate) odrl: String,
    pub(crate) org: String,
    pub(crate) owl: String,
    pub(crate) prof: String,
    pub(crate) prov: String,
    pub(crate) qb: String,
    pub(crate) rdf: String,
    pub(crate) rdfs: String,
    pub(crate) schema: String,
    pub(crate) sh: String,
    pub(crate) skos: String,
    pub(crate) sosa: String,
    pub(crate) ssn: String,
    pub(crate) time: String,
    pub(crate) vann: String,
    pub(crate) void: String,
    pub(crate) xsd: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct Id {
    #[serde(rename = "@id")]
    pub(crate) id: String,
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum RangeIncludes {
    Id(Id),
    Ids(Vec<Id>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum Type {
    Type(String),
    Types(Vec<String>),
}

impl Default for Type {
    fn default() -> Self {
        Type::Type(String::default())
    }
}

type DomainIncludes = RangeIncludes;
type SubclassOf = RangeIncludes;
type SubPropertyOf = RangeIncludes;
type EquivalentClass = RangeIncludes;
type Source = RangeIncludes;
type CloseMatch = RangeIncludes;

type Contributor = RangeIncludes;

impl Default for RangeIncludes {
    fn default() -> Self {
        RangeIncludes::Id(Id::default())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Graph {
    #[serde(rename = "@id")]
    pub(crate) id: String,
    #[serde(rename = "@type")]
    pub(crate) type_field: Type,
    #[serde(rename = "rdfs:comment")]
    pub(crate) rdfs_comment: TxtValue,
    #[serde(rename = "rdfs:label")]
    pub(crate) rdfs_label: TxtValue,
    #[serde(rename = "rdfs:subPropertyOf")]
    pub(crate) rdfs_sub_property_of: Option<SubPropertyOf>,
    #[serde(rename = "schema:domainIncludes")]
    pub(crate) schema_domain_includes: Option<DomainIncludes>,
    #[serde(rename = "schema:rangeIncludes")]
    pub(crate) schema_range_includes: Option<RangeIncludes>,
    #[serde(rename = "schema:isPartOf")]
    pub(crate) schema_is_part_of: Option<SchemaIsPartOf>,
    #[serde(rename = "rdfs:subClassOf")]
    pub(crate) rdfs_sub_class_of: Option<SubclassOf>,
    #[serde(rename = "schema:contributor")]
    pub(crate) schema_contributor: Option<Contributor>,
    #[serde(rename = "schema:source")]
    pub(crate) schema_source: Option<Source>,
    #[serde(rename = "owl:equivalentClass")]
    pub(crate) owl_equivalent_class: Option<EquivalentClass>,
    #[serde(rename = "schema:supersededBy")]
    pub(crate) schema_superseded_by: Option<SchemaSupersededBy>,
    #[serde(rename = "owl:equivalentProperty")]
    pub(crate) owl_equivalent_property: Option<OwlEquivalentProperty>,
    #[serde(rename = "skos:exactMatch")]
    pub(crate) skos_exact_match: Option<SkosExactMatch>,
    #[serde(rename = "schema:inverseOf")]
    pub(crate) schema_inverse_of: Option<SchemaInverseOf>,
    #[serde(rename = "schema:sameAs")]
    pub(crate) schema_same_as: Option<SchemaSameAs>,
    #[serde(rename = "skos:closeMatch")]
    pub(crate) skos_close_match: Option<CloseMatch>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SchemaIsPartOf {
    #[serde(rename = "@id")]
    pub(crate) id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SchemaSupersededBy {
    #[serde(rename = "@id")]
    pub(crate) id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OwlEquivalentProperty {
    #[serde(rename = "@id")]
    pub(crate) id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SkosExactMatch {
    #[serde(rename = "@id")]
    pub(crate) id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SchemaInverseOf {
    #[serde(rename = "@id")]
    pub(crate) id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SchemaSameAs {
    #[serde(rename = "@id")]
    pub(crate) id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum TxtValue {
    Txt(String),
    Translation {
        #[serde(rename = "@language")]
        language: String,
        #[serde(rename = "@value")]
        value: String,
    },
}

impl Default for TxtValue {
    fn default() -> Self {
        TxtValue::Txt(String::default())
    }
}

#[derive(Debug)]
pub(crate) struct PropertyDesc {
    pub(crate) _id: String,
    pub(crate) comment: String,
    pub(crate) label: String, // Name of the property camelCase for the type argument
    pub(crate) range_includes: HashSet<Id>, // Range of the property
    pub(crate) sub_properties: HashSet<Id>, // Sous propriétés
}

fn into_doc(source: &str) -> String {
    source.replace("[[", "[").replace("]]", "]").replace("\\n", "\n").replace('\n', "\n/// ")
}

impl PropertyDesc {
    pub(crate) fn doc(&self) -> String {
        into_doc(&self.comment)
    }
    
    pub(crate) fn feature_name(&self) -> String {
        format!("{}Prop", id_to_token(&self.label))
    }
}

#[derive(Debug)]
pub(crate) struct ClassDesc {
    pub(crate) label: String, // Name of the class PascalCase
    pub(crate) comment: String,
    pub(crate) sub_classes: HashSet<Id>,
    pub(crate) properties: HashSet<Id>, // Properties of the class
}

impl ClassDesc {
    pub(crate) fn doc(&self) -> String {
        into_doc(&self.comment)
    }

    pub(crate) fn feature_name(&self) -> String {
        id_to_token(&self.label)
    }

    pub(crate) fn cfg_feature(&self) -> String {
        match PRIMITIVE_TYPES.contains(&self.label.as_str()) {
            true => String::from("all()"),
            false => format!("feature = \"{}\"", self.feature_name()),
        }
    }
}

#[derive(Debug)]
pub(crate) struct SpecialTypeDesc {
    pub(crate) label: String, // Name of the class PascalCase
    pub(crate) comment: String,
    pub(crate) type_of: String, //  Sub classes
}

#[derive(Debug)]
pub(crate) struct Table {
    pub(crate) classes: HashMap<String, ClassDesc>,
    pub(crate) properties: HashMap<String, PropertyDesc>,
}

impl Table {
    pub(crate) fn from(schema: &Root) -> Table {
        let mut classes = HashMap::new();
        let mut properties = HashMap::new();
        let mut special_type = HashMap::new();

        // Add all existing elements to the table
        for node in &schema.graph {
            let id = node.id.clone();
            // Get common values
            let type_name = match &node.type_field {
                Type::Type(type_name) => type_name,
                Type::Types(types) => types.first().unwrap(),
            };

            if type_name.contains("Class") {
                let comment = match &node.rdfs_comment {
                    TxtValue::Txt(comment) => comment.to_string(),
                    TxtValue::Translation { value, .. } => value.to_string(),
                };

                let label = match &node.rdfs_label {
                    TxtValue::Txt(label) => label.to_string(),
                    TxtValue::Translation { value, .. } => value.to_string(),
                };

                let class = ClassDesc {
                    comment: comment.clone(),
                    label: label.clone(),
                    sub_classes: HashSet::new(),
                    properties: HashSet::new(),
                };
                classes.insert(id.to_string(), class);
            } else if type_name.contains("Property") {
                let comment = match &node.rdfs_comment {
                    TxtValue::Txt(comment) => comment.to_string(),
                    TxtValue::Translation { value, .. } => value.to_string(),
                };
                let label = match &node.rdfs_label {
                    TxtValue::Txt(label) => label.to_string(),
                    TxtValue::Translation { value, .. } => value.to_string(),
                };
                let range_includes = match &node.schema_range_includes {
                    Some(RangeIncludes::Id(id)) => {
                        vec![id.clone()]
                    }
                    Some(RangeIncludes::Ids(ids)) => ids.to_vec(),
                    None => Vec::new(),
                };
                let mut range_includes: HashSet<Id> = range_includes.into_iter().collect();
                range_includes.insert(Id {
                    id: "schema:Text".to_string(),
                });
                
                properties.insert(
                    id.clone(),
                    PropertyDesc {
                        _id: id,
                        comment: comment.clone(),
                        label: label.clone(),
                        range_includes,
                        sub_properties: HashSet::new(),
                    },
                );
            } else {
                let label = match &node.rdfs_label {
                    TxtValue::Txt(label) => label.to_string(),
                    TxtValue::Translation { value, .. } => value.to_string(),
                };
                let comment = match &node.rdfs_comment {
                    TxtValue::Txt(comment) => comment.to_string(),
                    TxtValue::Translation { value, .. } => value.to_string(),
                };
                special_type.insert(
                    label.clone(),
                    SpecialTypeDesc {
                        label,
                        comment,
                        type_of: type_name.to_string(),
                    }
                    .label,
                );
            }
        }

        // Fill the sub classes and properties
        for node in &schema.graph {
            let id = node.id.clone();

            // Class
            let type_name = match &node.type_field {
                Type::Type(type_name) => type_name,
                Type::Types(types) => types.first().unwrap(),
            };

            if type_name.contains("Class") {
                // Get the class
                let childs = match &node.rdfs_sub_class_of {
                    Some(SubclassOf::Id(id)) => {
                        vec![id.clone()]
                    }
                    Some(SubclassOf::Ids(ids)) => ids.to_vec(),
                    None => Vec::new(),
                };
                let class = if let Some(class) = classes.get_mut(&id) {
                    class
                } else {
                    println!("Class {} not found.", id);
                    continue;
                };

                class.sub_classes.extend(childs.into_iter());

                // Add the sub properties to property parents
                let childs = match &node.rdfs_sub_property_of {
                    Some(SubPropertyOf::Id(id)) => {
                        vec![id.clone()]
                    }
                    Some(SubPropertyOf::Ids(ids)) => ids.to_vec(),
                    None => Vec::new(),
                };
                class.properties.extend(childs.into_iter());
            } else if type_name.contains("Property") {
                let childs = match &node.rdfs_sub_property_of {
                    Some(SubPropertyOf::Id(id)) => {
                        vec![id.clone()]
                    }
                    Some(SubPropertyOf::Ids(ids)) => ids.to_vec(),
                    None => Vec::new(),
                };

                let property = if let Some(property) = properties.get_mut(&id) {
                    property
                } else {
                    println!("Property {} not found.", id);
                    continue;
                };
                property.sub_properties = childs.into_iter().collect();

                // Add the properties to classes
                let classes_with_prop = match &node.schema_domain_includes {
                    Some(DomainIncludes::Id(id)) => {
                        vec![id.clone()]
                    }
                    Some(DomainIncludes::Ids(ids)) => ids.to_vec(),
                    None => Vec::new(),
                };

                for class in &classes_with_prop {
                    let class = if let Some(class) = classes.get_mut(&class.id) {
                        class
                    } else {
                        println!("Class {:?} not found for {}", class, id);
                        continue;
                    };

                    class.properties.insert(Id { id: id.to_owned() });
                }
            } else {
                let label = match &node.rdfs_label {
                    TxtValue::Txt(label) => label.to_string(),
                    TxtValue::Translation { value, .. } => value.to_string(),
                };
                special_type.insert(id.to_string(), label);
            }
        }

        for primitive in crate::writer::PRIMITIVE_TYPES {
            classes.insert(
                format!("schema:{}", primitive),
                ClassDesc {
                    comment: format!("Primitive type {}", primitive),
                    label: primitive.to_string(),
                    sub_classes: HashSet::new(),
                    properties: HashSet::new(),
                },
            );
        }

        // Replace all URL by Url
        for class in classes.values_mut() {
            if class.properties.remove(&Id { id: "schema:URL".to_string() }) {
                class.properties.insert(Id { id: "schema:Url".to_string() });
            }
        }
        for property in properties.values_mut() {
            if property.range_includes.remove(&Id { id: "schema:URL".to_string() }) {
                property.range_includes.insert(Id { id: "schema:Url".to_string() });
            }
        }

        Self {
            classes,
            properties,
        }
    }
}

pub(crate) fn read_schema() -> Table {
    let file = std::fs::File::open("schemaorg.jsonld").unwrap();
    let reader = std::io::BufReader::new(file);
    let root: Root = serde_json::from_reader(reader).unwrap();
    Table::from(&root)
}
