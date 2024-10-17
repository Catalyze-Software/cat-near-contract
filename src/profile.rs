use crate::error::GenericError;
use crate::models::profile::{PostProfile, ProfileResponse, UpdateProfile};
use crate::models::response_result::ResponseResult;
use crate::models::rewards::Rewards;
use crate::{Contract, ContractExt};
use near_sdk::{env, near, AccountId};

//Nice to have
//add validation to PostProfile and UpdateProfile

#[near]
impl Contract {
    pub fn add_profile(&mut self, post_profile: PostProfile) -> ResponseResult<ProfileResponse> {
        let account_id = env::signer_account_id();
        match self
            .profiles
            .insert(account_id.clone(), post_profile.into())
        {
            Some(_) => ResponseResult::Err(GenericError::ProfileAlreadyExists),
            None => {
                env::log_str("Profile added");
                ResponseResult::Ok(ProfileResponse::new(
                    account_id.clone(),
                    self.profiles.get(&account_id).unwrap().clone(),
                ))
            }
        }
    }

    // pub fn edit_profile(
    //     &mut self,
    //     update_profile: UpdateProfile,
    // ) -> ResponseResult<ProfileResponse> {
    //     let account_id = env::signer_account_id();
    //     match self.profiles.get_mut(&account_id) {
    //         None => ResponseResult::Err(GenericError::ProfileNotFound),
    //         Some(profile) => {
    //             profile.update(update_profile);

    //             if profile.is_filled() {
    //                 match self.rewards.get_mut(&account_id) {
    //                     Some(reward) => {
    //                         reward.profile_complete();
    //                     }
    //                     None => {
    //                         let mut new_reward = Rewards::default();
    //                         new_reward.profile_complete();
    //                         self.rewards.insert(account_id.clone(), new_reward);
    //                     }
    //                 };
    //             };
    //             env::log_str("Profile updated");
    //             ResponseResult::Ok(ProfileResponse::new(account_id, profile.clone()))
    //         }
    //     }
    // }

    pub fn edit_profile(
        &mut self,
        update_profile: UpdateProfile,
    ) -> ResponseResult<ProfileResponse> {
        let account_id = env::signer_account_id();
        match self.profiles.get(&account_id) {
            None => ResponseResult::Err(GenericError::ProfileNotFound),
            Some(profile) => {
                let updated_profile = profile.update(update_profile);

                self.profiles
                    .insert(account_id.clone(), updated_profile.clone());

                if updated_profile.is_filled() {
                    match self.rewards.get(&account_id) {
                        Some(reward) => {
                            let mut x = reward.clone();
                            x.profile_complete();
                            self.rewards.insert(account_id.clone(), x);
                        }
                        None => {
                            let mut new_reward = Rewards::default();
                            new_reward.profile_complete();
                            self.rewards.insert(account_id.clone(), new_reward);
                        }
                    };
                };

                env::log_str("Profile updated");
                ResponseResult::Ok(ProfileResponse::new(account_id, updated_profile.clone()))
            }
        }
    }

    pub fn get_profile(&self, account_id: AccountId) -> ResponseResult<ProfileResponse> {
        self.profiles.get(&account_id).map_or_else(
            || ResponseResult::Err(GenericError::ProfileNotFound),
            |profile| ResponseResult::Ok(ProfileResponse::new(account_id, profile.clone())),
        )
    }

    pub fn get_profiles(&self, account_ids: Vec<AccountId>) -> Vec<ProfileResponse> {
        let mut profiles: Vec<ProfileResponse> = vec![];
        for account_id in &account_ids {
            if let Some(_profile) = self.profiles.get(account_id) {
                profiles.push(ProfileResponse::new(account_id.clone(), _profile.clone()));
            }
        }
        profiles
    }
}
