// Find all our documentation at https://docs.near.org
pub mod error;
pub mod groups;
pub mod models;
pub mod profile;

use crate::models::groups::GroupWithMembers;
use crate::models::profile::Profile;
use near_sdk::{near, store::IterableMap, AccountId};

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    pub profiles: IterableMap<AccountId, Profile>,
    pub groups: IterableMap<u32, GroupWithMembers>,
    pub group_id_counter: u32,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            profiles: IterableMap::new(b"p"),
            groups: IterableMap::new(b"g"),
            group_id_counter: 0,
        }
    }
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            profiles: IterableMap::new(b"p"),
            groups: IterableMap::new(b"g"),
            group_id_counter: 0,
        }
    }
}
