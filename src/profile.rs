//need gas checks?
use crate::models::profile::{PostProfile, ProfileResponse, UpdateProfile};
use crate::{Contract, ContractExt};
use near_sdk::{env, near, AccountId};

#[near]
impl Contract {
    pub fn add_profile(&mut self, post_profile: PostProfile) {
        let account_id = env::signer_account_id();
        self.profiles.insert(account_id, post_profile.into());
        env::log_str("Profile added");
    }

    pub fn edit_profile(&mut self, update_profile: UpdateProfile) {
        let account_id = env::signer_account_id();
        let current_profile = self.profiles.get(&account_id).expect("Profile not found");

        //instead of cloning the while current profile here, only clone internally what is needed.
        let updated_profile = current_profile.update(update_profile);
        self.profiles.insert(account_id, updated_profile);
        env::log_str(&format!("Profile updated"));
    }

    pub fn get_profile(&self, account_id: AccountId) -> Option<ProfileResponse> {
        let profile = self
            .profiles
            .get(&account_id)
            .expect("Profile not found for the given account ID");

        Some(ProfileResponse::from((account_id, profile.clone())))
    }

    pub fn get_profiles(&self, account_ids: Vec<AccountId>) -> Vec<ProfileResponse> {
        account_ids
            .into_iter()
            .map(|account_id| {
                let profile = self
                    .profiles
                    .get(&account_id)
                    .expect("Profile not found for the given account ID");

                ProfileResponse::from((account_id, profile.clone()))
            })
            .collect()
    }
}
