use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod projectconfig;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct AccountId(pub i32);

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct Weights {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub accounts: HashMap<AccountId, u32>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub projects: HashMap<String, u32>,
}
