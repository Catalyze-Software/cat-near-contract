use near_sdk::{env, AccountId};
use serde::{Deserialize, Serialize};

use crate::models::members::Members;

use super::application_role::ApplicationRole;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GroupWithMembers {
    pub name: String,
    pub description: String,
    pub website: String,
    pub image: String,        //url to the IPFS image
    pub banner_image: String, //url to the IPFS image,
    pub owner: AccountId,
    pub created_by: AccountId,
    pub members: Members,
    pub matrix_space_id: String,
    pub is_deleted: bool,
    pub updated_on: u64,
    pub created_on: u64,
}

impl Default for GroupWithMembers {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            website: Default::default(),
            image: Default::default(),
            banner_image: Default::default(),
            owner: env::signer_account_id(),
            created_by: env::signer_account_id(),
            members: Default::default(),
            is_deleted: Default::default(),
            updated_on: Default::default(),
            created_on: Default::default(),
            matrix_space_id: Default::default(),
        }
    }
}

impl From<PostGroup> for GroupWithMembers {
    fn from(group: PostGroup) -> Self {
        Self {
            name: group.name,
            description: group.description,
            website: group.website,
            image: group.image,
            banner_image: group.banner_image,
            owner: env::signer_account_id(),
            created_by: env::signer_account_id(),
            members: Members::new_with_owner(env::signer_account_id()),
            is_deleted: false,
            updated_on: env::block_timestamp(),
            created_on: env::block_timestamp(),
            matrix_space_id: group.matrix_space_id,
        }
    }
}

impl GroupWithMembers {
    pub fn update(&mut self, group: UpdateGroup) -> Self {
        self.name = group.name;
        self.description = group.description;
        self.website = group.website;
        self.image = group.image;
        self.banner_image = group.banner_image;
        self.updated_on = env::block_timestamp();
        self.clone()
    }

    pub fn set_owner(&mut self, owner: AccountId) -> Self {
        self.owner = owner.clone();
        self.members.set_owner(owner);
        self.clone()
    }

    pub fn delete(&mut self) -> Self {
        self.is_deleted = true;
        self.updated_on = env::block_timestamp();
        self.clone()
    }

    pub fn get_members(&self) -> Vec<AccountId> {
        self.members.members.keys().cloned().collect()
    }

    pub fn remove_member(&mut self, member: AccountId) {
        self.members.members.remove(&member);
    }

    pub fn add_member(&mut self, member: AccountId) {
        self.members.members.insert(member, ApplicationRole::Member);
    }

    pub fn is_member(&self, member: AccountId) -> bool {
        self.members.is_member(member)
    }
}

pub type GroupEntry = (u64, GroupWithMembers);

#[derive(Clone, Deserialize)]
pub struct PostGroup {
    pub name: String,
    pub description: String,
    pub website: String,
    pub matrix_space_id: String,
    pub image: String,
    pub banner_image: String,
    pub tags: Vec<u32>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct UpdateGroup {
    pub name: String,
    pub description: String,
    pub website: String,
    pub image: String,
    pub banner_image: String,
    pub tags: Vec<u32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GroupResponse {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub website: String,
    pub created_by: AccountId,
    pub owner: AccountId,
    pub matrix_space_id: String,
    pub image: String,
    pub banner_image: String,
    pub is_deleted: bool,
    pub updated_on: u64,
    pub created_on: u64,
    pub members_count: u64,
}

impl GroupResponse {
    pub fn new(id: u64, group: GroupWithMembers) -> Self {
        Self {
            id,
            name: group.name,
            description: group.description,
            website: group.website,
            created_by: group.created_by,
            owner: group.owner,
            matrix_space_id: group.matrix_space_id,
            image: group.image,
            banner_image: group.banner_image,
            is_deleted: group.is_deleted,
            updated_on: group.updated_on,
            created_on: group.created_on,
            members_count: group.members.members.len() as u64,
        }
    }
}
