use serde::{Deserialize, Serialize};

pub mod projectconfig;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct AccountId(pub i32);

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
