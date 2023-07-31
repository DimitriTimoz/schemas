use crate::types::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

include!(concat!(env!("OUT_DIR"), "/properties.rs"));
