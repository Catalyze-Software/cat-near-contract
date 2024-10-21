use near_sdk::{env, near, AccountId};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use super::application_role::ApplicationRole;

#[derive(Clone, Debug, Default)]
#[near(serializers = ["json", "borsh"])]
pub struct Members {
    pub members: HashMap<AccountId, ApplicationRole>,
}

impl Members {
    pub fn new_with_owner(owner: AccountId) -> Self {
        let mut members = HashMap::new();
        members.insert(owner, ApplicationRole::Owner);
        Self { members }
    }

    pub fn set_owner(&mut self, new_owner: AccountId) -> bool {
        // Set new owner
        match self.members.entry(new_owner.clone()) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() = ApplicationRole::Owner;
                env::log_str(&format!("Existing member {} promoted to owner", new_owner));
                true
            }
            Entry::Vacant(entry) => {
                entry.insert(ApplicationRole::Owner);
                env::log_str(&format!("New owner {} added", new_owner));
                true
            }
        }
    }

    pub fn is_member(&self, member: &AccountId) -> bool {
        self.members.contains_key(member)
    }
}
