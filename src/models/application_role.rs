use core::fmt;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    BorshDeserialize,
    BorshSerialize,
)]
#[borsh(crate = "near_sdk::borsh")]
pub enum ApplicationRole {
    Owner,
    #[default]
    Member,
}

impl fmt::Display for ApplicationRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ApplicationRole::*;
        match self {
            Owner => write!(f, "Owner"),
            Member => write!(f, "Member"),
        }
    }
}
