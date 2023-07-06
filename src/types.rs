use crate::properties::*;

#[derive(Debug, Clone, Default)]
pub struct Text(String);

#[derive(Debug, Clone, Default)]
pub struct Number(f64);

#[derive(Debug, Clone, Default)]
pub struct Integer(i64);

#[derive(Debug, Clone, Default)]
pub struct Boolean(bool);

#[derive(Debug, Clone, Default)]
pub struct Date(String);

#[derive(Debug, Clone, Default)]
pub struct DateTime(String);

#[derive(Debug, Clone, Default)]
pub struct URL(Text);

#[derive(Debug, Clone, Default)]
pub struct Time(String);

#[derive(Debug, Clone, Default)]
pub struct XPathType {
    sub_class: Text,
}

#[derive(Debug, Clone, Default)]
pub struct CssSelectorType {
    sub_class: Text,
}

pub trait Schema {
    fn new() -> Self;
    fn add_property(&mut self, name: String, value: String);
}

include!(concat!(env!("OUT_DIR"), "/types.rs"));
