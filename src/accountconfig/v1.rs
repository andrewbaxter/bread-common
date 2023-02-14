use std::collections::HashMap;
use serde::{
    Deserialize,
    Serialize,
};
use crate::AccountId;

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct AccountDest {
    pub weight: u32,
    pub memo: String,
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct Weights {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub accounts: HashMap<AccountId, AccountDest>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub projects: HashMap<String, u32>,
}
