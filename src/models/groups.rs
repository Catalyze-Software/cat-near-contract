use super::application_role::ApplicationRole;
use crate::models::members::Members;
use near_sdk::{env, near, AccountId};

#[derive(Clone, Debug)]
#[near(serializers = ["json", "borsh"])]
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
            owner: env::predecessor_account_id(),
            created_by: env::predecessor_account_id(),
            members: Default::default(),
            is_deleted: Default::default(),
            updated_on: Default::default(),
            created_on: Default::default(),
            matrix_space_id: Default::default(),
        }
    }
}

#[derive(Clone)]
#[near(serializers = ["json"])]
pub struct PostGroup {
    pub name: String,
    pub description: String,
    pub website: String,
    pub matrix_space_id: String,
    pub image: String,
    pub banner_image: String,
    pub tags: Vec<u32>,
}

impl From<PostGroup> for GroupWithMembers {
    fn from(group: PostGroup) -> Self {
        Self {
            name: group.name,
            description: group.description,
            website: group.website,
            image: group.image,
            banner_image: group.banner_image,
            owner: env::predecessor_account_id(),
            created_by: env::predecessor_account_id(),
            members: Members::new_with_owner(env::predecessor_account_id()),
            is_deleted: false,
            updated_on: env::block_timestamp(),
            created_on: env::block_timestamp(),
            matrix_space_id: group.matrix_space_id,
        }
    }
}

#[derive(Clone, Debug)]
#[near(serializers = ["json"])]
pub struct UpdateGroup {
    pub name: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub image: Option<String>,
    pub banner_image: Option<String>,
    pub tags: Option<Vec<u32>>,
}

impl GroupWithMembers {
    pub fn update(&mut self, group: UpdateGroup) {
        self.name = group.name.unwrap_or_else(|| self.name.clone());
        self.description = group
            .description
            .unwrap_or_else(|| self.description.clone());
        self.website = group.website.unwrap_or_else(|| self.website.clone());
        self.image = group.image.unwrap_or_else(|| self.image.clone());
        self.banner_image = group
            .banner_image
            .unwrap_or_else(|| self.banner_image.clone());
        self.owner = self.owner.clone();
        self.created_by = self.created_by.clone();
        self.members = self.members.clone();
        self.updated_on = env::block_timestamp();
        self.matrix_space_id = self.matrix_space_id.clone();
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

    pub fn remove_member(&mut self, member: AccountId) -> Self {
        self.members.members.remove(&member);
        self.clone()
    }

    pub fn get_member_with_role(&self, member: &AccountId) -> Option<(AccountId, ApplicationRole)> {
        self.members
            .members
            .iter()
            .find(|(k, _)| k == &member)
            .map(|(k, v)| (k.clone(), v.clone()))
    }

    pub fn get_members_with_roles(&self) -> Vec<(AccountId, ApplicationRole)> {
        self.members
            .members
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn add_member(&mut self, member: AccountId) -> Self {
        self.members.members.insert(member, ApplicationRole::Member);
        self.clone()
    }

    pub fn is_member(&self, member: &AccountId) -> bool {
        self.members.is_member(member)
    }
}

#[derive(Clone, Debug)]
#[near(serializers = ["json"])]
pub struct GroupResponse {
    pub id: u32,
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
    pub fn new(id: u32, group: GroupWithMembers) -> Self {
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
