use crate::properties::*;

#[derive(Debug, Clone)]
pub struct Text(String);

#[derive(Debug, Clone)]
pub struct Number(f64);

#[derive(Debug, Clone)]
pub struct Boolean(bool);

#[derive(Debug, Clone)]
pub struct Date(String);

#[derive(Debug, Clone)]
pub struct DateTime(String);

#[derive(Debug, Clone)]
pub struct Url(String);

#[derive(Debug, Clone)]
pub struct Time(String);

include!(concat!(env!("OUT_DIR"), "/types.rs"));