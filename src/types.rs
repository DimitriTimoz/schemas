use crate::properties::*;

#[derive(Debug, Clone)]
pub struct Text(String);

#[derive(Debug, Clone)]
pub struct Number(f64);

#[derive(Debug, Clone)]
pub struct Integer(i64);

#[derive(Debug, Clone)]
pub struct Boolean(bool);

#[derive(Debug, Clone)]
pub struct Date(String);

#[derive(Debug, Clone)]
pub struct DateTime(String);

#[derive(Debug, Clone)]
pub struct URL(Text);

#[derive(Debug, Clone)]
pub struct Time(String);

#[derive(Debug, Clone)]
pub struct XPathType {
    sub_class: Text,
}

#[derive(Debug, Clone)]
pub struct CssSelectorType {
    sub_class: Text,
}

include!(concat!(env!("OUT_DIR"), "/types.rs"));