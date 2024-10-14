use crate::models::groups::GroupWithMembers;
use crate::models::profile::Profile;
use near_sdk::{near, store::IterableMap, AccountId};

use super::reward::Rewards;

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    pub profiles: IterableMap<AccountId, Profile>,
    pub groups: IterableMap<u32, GroupWithMembers>,
    pub rewards: IterableMap<AccountId, Rewards>,
    pub group_id_counter: u32,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            profiles: IterableMap::new(b"p"),
            groups: IterableMap::new(b"g"),
            rewards: IterableMap::new(b"r"),
            group_id_counter: 0,
        }
    }
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self::default()
    }
}
