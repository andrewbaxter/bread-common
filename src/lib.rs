use std::collections::{
    BTreeMap,
    HashMap,
};
use serde::{
    Deserialize,
    Serialize,
    Serializer,
};

pub mod projectconfig;
pub mod accountconfig;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct AccountId(pub i64);

impl AccountId {
    pub fn from_sql(x: i64) -> Result<Self, String> {
        Ok(AccountId(x))
    }

    pub fn to_sql(self) -> i64 {
        self.0
    }
}

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

pub fn serde_order_map<
    K: Ord + Serialize,
    V: Serialize,
    S: serde::Serializer,
>(value: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer {
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

pub const DEFAULT_WEIGHT: u32 = 100;
