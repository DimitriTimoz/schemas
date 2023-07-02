use std::collections::{HashMap, HashSet};

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeName {
    // Generate
    Name,
    Book,
    Movie
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropertyName {
    // Generate
    Url,
}


pub trait ItemType  {
    fn get_nomalised_properties(&self) -> HashSet<PropertyName> where Self: Sized;
    fn get_properties(&self) ->  &HashMap<TypeName, Box<(dyn PropertyType + 'static)>> where Self: Sized;
    fn get_property(&self, key: &TypeName) -> Option<&(dyn PropertyType + 'static)>  where Self: Sized;
    fn set_property(&mut self, key: TypeName, value: Box<dyn PropertyType>) where Self: Sized;
}

pub trait PropertyType: ItemType + PropertyTrait {}

/*pub struct Item {
    types: HashSet<TypeName>,
    properties: HashMap<PropertyName, Box<dyn PropertyType>>
}*/


pub trait PropertyTrait {
    fn get_range() -> &'static HashSet<TypeName> where Self: Sized;
    fn get_domain() -> &'static HashSet<TypeName> where Self: Sized;
}



struct Movie {
    properties: HashMap<TypeName, Box<dyn PropertyType>>,
    types: HashSet<TypeName>,
}

impl ItemType for Movie {
    fn get_nomalised_properties(&self) -> HashSet<PropertyName> where Self: Sized {
        HashSet::<PropertyName>::from_iter(vec![PropertyName::Url].iter().cloned())
    }

    fn get_properties(&self) -> &HashMap<TypeName, Box<(dyn PropertyType + 'static)>> where Self: Sized {
        &self.properties
    }

    fn get_property(&self, key: &TypeName) -> Option<&(dyn PropertyType + 'static)>  where Self: Sized {
        self.properties.get(key).map(|boxed| &**boxed)
    }

    fn set_property(&mut self, key: TypeName, value: Box<dyn PropertyType>) where Self: Sized {
        self.properties.insert(key, value);
    }
}


include!(concat!(env!("OUT_DIR"), "/gen.rs"));
