use crate::{
    error::GenericError,
    models::{
        application_role::ApplicationRole,
        groups::{GroupResponse, PostGroup, UpdateGroup},
        response_result::ResponseResult,
        rewards::Rewards,
    },
    Contract, ContractExt,
};

use near_sdk::{env, near, AccountId};

#[near]
impl Contract {
    pub fn add_group(&mut self, post_group: PostGroup) -> ResponseResult<GroupResponse> {
        let account_id = env::predecessor_account_id();

        let group_id = self.group_id_counter;
        match self.profiles.get_mut(&account_id) {
            None => ResponseResult::Err(GenericError::ProfileNotFound),

            Some(profile) => {
                match self.groups.insert(group_id, post_group.into()) {
                    Some(_) => ResponseResult::Err(GenericError::GroupNotAdded),
                    None => {
                        profile.add_group(group_id);

                        env::log_str(&format!("Group added with id {}", group_id));
                        let group = self.groups.get(&group_id).unwrap(); // safely unwrap since we just inserted
                        self.group_id_counter += 1;

                        ResponseResult::Ok(GroupResponse::new(group_id, group.clone()))
                    }
                }
            }
        }
    }

    pub fn edit_group(
        &mut self,
        id: u32,
        update_group: UpdateGroup,
    ) -> ResponseResult<GroupResponse> {
        match self.groups.get_mut(&id) {
            None => ResponseResult::Err(GenericError::GroupNotFound),
            Some(group) => {
                group.update(update_group);

                env::log_str(&format!("Group {} updated", id));
                ResponseResult::Ok(GroupResponse::new(id, group.clone()))
            }
        }
    }

    pub fn get_group(&self, id: u32) -> ResponseResult<GroupResponse> {
        match self.groups.get(&id) {
            None => ResponseResult::Err(GenericError::GroupNotFound),
            Some(group) => ResponseResult::Ok(GroupResponse::new(id, group.clone())),
        }
    }

    pub fn get_group_by_name(&self, name: String) -> ResponseResult<GroupResponse> {
        self.groups
            .iter()
            .find(|(_, group)| group.name.to_lowercase() == name.to_lowercase())
            .map(|(id, group)| ResponseResult::Ok(GroupResponse::new(*id, group.clone())))
            .unwrap_or(ResponseResult::Err(GenericError::GroupNotFound))
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

    pub fn join_group(&mut self, group_id: u32) -> ResponseResult<GroupResponse> {
        let account_id = env::predecessor_account_id();

        match self.profiles.get_mut(&account_id) {
            None => ResponseResult::Err(GenericError::ProfileNotFound),
            Some(profile) => {
                if profile.is_group_member(&group_id) {
                    return ResponseResult::Err(GenericError::AlreadyMember);
                }

                match self.groups.get_mut(&group_id) {
                    None => ResponseResult::Err(GenericError::GroupNotFound),
                    Some(group) => {
                        if group.is_member(&account_id) {
                            return ResponseResult::Err(GenericError::UserAlreadyInGroup);
                        }

                        group.add_member(account_id.clone());
                        profile.add_group(group_id);

                        match self.rewards.get_mut(&account_id) {
                            Some(reward) => {
                                reward.group_join(group_id);
                            }
                            None => {
                                self.rewards.insert(
                                    account_id.clone(),
                                    Rewards::default().group_join(group_id),
                                );
                            }
                        };

                        env::log_str(&format!("User {} joined group {}", account_id, group_id));
                        ResponseResult::Ok(GroupResponse::new(group_id, group.clone()))
                    }
                }
            }
        }
    }

    pub fn leave_group(&mut self, group_id: u32) -> ResponseResult<()> {
        let account_id = env::predecessor_account_id();

        match self.profiles.get_mut(&account_id) {
            None => ResponseResult::Err(GenericError::ProfileNotFound),
            Some(profile) => {
                if !profile.is_group_member(&group_id) {
                    return ResponseResult::Err(GenericError::NotMember);
                }

                profile.remove_group(group_id);

                match self.groups.get_mut(&group_id) {
                    None => ResponseResult::Err(GenericError::GroupNotFound),
                    Some(group) => {
                        if !group.is_member(&account_id) {
                            return ResponseResult::Err(GenericError::NotMember);
                        }
                        group.remove_member(account_id.clone());

                        env::log_str(&format!("User {} joined group {}", account_id, group_id));
                        ResponseResult::Ok(())
                    }
                }
            }
        }
    }

    pub fn get_user_groups(&self, account_id: AccountId) -> Vec<u32> {
        self.profiles
            .get(&account_id)
            .map(|profile| profile.get_group_ids())
            .unwrap_or_default()
    }

    pub fn is_user_in_group(&self, account_id: AccountId, group_id: u32) -> bool {
        self.groups
            .get(&group_id)
            .map(|group| group.is_member(&account_id))
            .unwrap_or(false)
    }

    pub fn get_group_members(&self, group_id: u32) -> Vec<(AccountId, ApplicationRole)> {
        self.groups
            .get(&group_id)
            .map(|group| group.get_members_with_roles())
            .unwrap_or_default()
    }

    pub fn get_user_role_in_group(
        &self,
        account_id: AccountId,
        group_id: u32,
    ) -> Option<(AccountId, ApplicationRole)> {
        self.groups
            .get(&group_id)
            .and_then(|group| group.get_member_with_role(&account_id))
    }
}
