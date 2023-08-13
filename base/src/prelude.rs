pub use crate::properties::*;
pub use crate::traits::*;
pub use crate::types::*;
pub use crate::classes::*;
pub use crate::value::*;

#[cfg(feature = "serde")]
pub(crate) use serde::{Deserialize, Serialize};

pub(crate) use std::collections::HashMap;
pub(crate) static EMPTY_VEC: Vec<SchemaValue> = Vec::new();
