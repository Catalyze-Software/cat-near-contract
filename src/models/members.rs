use near_sdk::{env, AccountId};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use super::application_role::ApplicationRole;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
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

    pub fn is_member(&self, member: AccountId) -> bool {
        self.members.contains_key(&member)
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct Join {
//     pub roles: Vec<String>,
//     pub updated_at: u64,
//     pub created_at: u64,
// }

// impl Default for Join {
//     fn default() -> Self {
//         Self {
//             roles: vec![MEMBER_ROLE.into()],
//             updated_at: env::block_timestamp(),
//             created_at: env::block_timestamp(),
//         }
//     }
// }

// impl Join {
//     pub fn set_owner_role(&mut self) -> Self {
//         Self {
//             roles: vec![(OWNER_ROLE.into())],
//             updated_at: env::block_timestamp(),
//             created_at: env::block_timestamp(),
//         }
//     }

//     pub fn has_owner_role(&self) -> bool {
//         self.roles.contains(&OWNER_ROLE.into())
//     }

//     pub fn set_member_role(&mut self) -> Self {
//         Self {
//             roles: vec![(MEMBER_ROLE.into())],
//             updated_at: env::block_timestamp(),
//             created_at: env::block_timestamp(),
//         }
//     }

//     pub fn has_member_role(&self) -> bool {
//         self.roles.contains(&MEMBER_ROLE.into())
//     }

//     // pub fn set_role(&mut self, role: String) {
//     //     if ![MEMBER_ROLE].contains(&role.as_str()) {
//     //         return;
//     //     }
//     //     self.roles = vec![role];
//     //     self.updated_at = env::block_timestamp();
//     // }

//     pub fn remove_role(&mut self, role: String) {
//         self.roles.retain(|r| r != &role);
//         self.updated_at = env::block_timestamp();
//     }
// }
