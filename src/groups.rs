use crate::error::GroupError;
use crate::models::{
    application_role::ApplicationRole,
    groups::{GroupResponse, PostGroup, UpdateGroup},
};
use crate::{Contract, ContractExt};
use near_sdk::{env, near, AccountId};
use std::collections::HashMap;

#[near]
impl Contract {
    #[handle_result]
    pub fn add_group(&mut self, post_group: PostGroup) -> Result<u32, GroupError> {
        let account_id = env::predecessor_account_id();

        // Update Profile
        let profile = self
            .profiles
            .get_mut(&account_id)
            .ok_or(GroupError::ProfileNotFound)?;

        let group_id = self.group_id_counter;

        profile.joined_groups.push(group_id);

        self.groups.insert(group_id, post_group.into());
        self.group_id_counter += 1;
        env::log_str((format!("Group added with id {}", group_id)).as_str());
        Ok(group_id)
    }

    pub fn edit_group(&mut self, id: u32, update_group: UpdateGroup) -> Option<()> {
        let current_group = self.groups.get(&id)?;
        //instead of cloning the whole current group here, only clone internally what is needed.
        let updated_group = current_group.update(update_group);
        self.groups.insert(id, updated_group);
        env::log_str(format!("Group {} updated", id).as_str());
        Some(())
    }

    pub fn get_group(&self, id: u32) -> Option<GroupResponse> {
        let group = self.groups.get(&id)?;
        Some(GroupResponse::new(id, group.clone()))
    }

    pub fn get_group_by_name(&self, name: String) -> Option<GroupResponse> {
        self.groups
            .iter()
            .find(|(_, group)| group.name == name)
            .map(|(id, group)| GroupResponse::new(*id, group.clone()))
    }

    pub fn get_groups(&self, index: u32, limit: u32) -> Vec<GroupResponse> {
        let mut groups: Vec<GroupResponse> = vec![];
        for (id, group) in self.groups.iter().skip(index as _).take(limit as _) {
            groups.push(GroupResponse::new(*id, group.clone()));
        }
        groups
    }

    pub fn get_groups_by_id(&self, ids: Vec<u32>) -> Vec<GroupResponse> {
        let mut groups: Vec<GroupResponse> = vec![];
        for id in &ids {
            if let Some(_group) = self.groups.get(id) {
                groups.push(GroupResponse::new(*id, _group.clone()));
            }
        }
        groups
    }

    //Don't return Okay for Errors.
    #[handle_result]
    pub fn join_group(&mut self, group_id: u32) -> Result<(), GroupError> {
        let account_id = env::predecessor_account_id();

        // Update Profile
        let profile = self
            .profiles
            .get_mut(&account_id)
            .ok_or(GroupError::ProfileNotFound)?;

        if profile.joined_groups.contains(&group_id) {
            return Err(GroupError::AlreadyMember);
        }
        profile.joined_groups.push(group_id);

        // Update Group
        let group = self
            .groups
            .get_mut(&group_id)
            .ok_or(GroupError::GroupNotFound)?;

        if group.members.members.contains_key(&account_id) {
            return Err(GroupError::UserAlreadyInGroup);
        }
        group
            .members
            .members
            .insert(account_id, ApplicationRole::Member);

        Ok(())
    }

    #[handle_result]
    pub fn leave_group(&mut self, group_id: u32) -> Result<(), GroupError> {
        let account_id = env::predecessor_account_id();

        // Update Profile
        // Update Profile
        let profile = self
            .profiles
            .get_mut(&account_id)
            .ok_or(GroupError::ProfileNotFound)?;

        if !profile.joined_groups.contains(&group_id) {
            return Err(GroupError::NotMember);
        }

        profile.joined_groups.retain(|&x| x != group_id);

        // Update Group
        let group = self
            .groups
            .get_mut(&group_id)
            .ok_or(GroupError::GroupNotFound)?;

        if !group.members.members.contains_key(&account_id) {
            return Err(GroupError::NotMember);
        }
        group.members.members.remove(&account_id);

        Ok(())
    }

    pub fn get_user_groups(&self, account_id: AccountId) -> Vec<u32> {
        self.profiles
            .get(&account_id)
            .map(|profile| profile.joined_groups.clone())
            .unwrap_or_default()
    }

    pub fn is_user_in_group(&self, account_id: AccountId, group_id: u32) -> bool {
        self.groups
            .get(&group_id)
            .map(|group| group.members.members.contains_key(&account_id))
            .unwrap_or(false)
    }

    pub fn get_group_members(&self, group_id: u32) -> HashMap<AccountId, ApplicationRole> {
        self.groups
            .get(&group_id)
            .map(|group| group.members.members.clone())
            .unwrap_or_default()
    }

    pub fn get_user_role_in_group(
        &self,
        account_id: AccountId,
        group_id: u32,
    ) -> Option<ApplicationRole> {
        self.groups
            .get(&group_id)
            .and_then(|group| group.members.members.get(&account_id).cloned())
    }
}
