use serde::{
    Deserialize,
    Serialize,
};

pub mod v1;

pub mod latest {
    pub use super::v1::*;
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum VersionedProjectConfig {
    V1(v1::Config),
}

impl VersionedProjectConfig {
    pub fn from_sql(x: Vec<u8>) -> Result<Self, serde_json::Error> {
        Ok(serde_json::from_slice(&x)?)
    }

    pub fn to_sql(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

pub const FILENAME: &'static str = ".bread.yml";
