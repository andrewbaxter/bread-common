use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{is_default, AccountId};

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct Weights {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub accounts: HashMap<AccountId, u32>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub projects: HashMap<String, u32>,
}

impl Weights {
    pub fn to_common(self) -> crate::Weights {
        crate::Weights {
            accounts: self.accounts,
            projects: self.projects,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct Config {
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
    pub weights: Weights,
}