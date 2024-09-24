// Find all our documentation at https://docs.near.org
pub mod models;
pub mod profile;

use crate::models::profile::Profile;
use near_sdk::{near, store::IterableMap, AccountId};

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    pub profiles: IterableMap<AccountId, Profile>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            profiles: IterableMap::new(b"p"),
        }
    }
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            profiles: IterableMap::new(b"p"),
        }
    }
}
