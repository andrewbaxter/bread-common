use serde::{Deserialize, Serialize};

pub mod v1;
pub mod latest {
    pub use super::v1::*;
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum VersionedProjectConfig {
    V1(v1::Config),
}

pub const FILENAME: &'static str = ".bread.yml";
