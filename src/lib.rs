use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize, Serializer};

pub mod projectconfig;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct AccountId(pub i32);

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

pub fn serde_order_map<K: Ord + Serialize, V: Serialize, S: serde::Serializer>(
    value: &HashMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}
