use crate::{models::rewards::Rewards, Contract, ContractExt};
use near_sdk::{near, AccountId};

#[near]
impl Contract {
    pub fn get_rewards(&self, account_id: AccountId) -> Rewards {
        match self.rewards.get(&account_id) {
            Some(reward) => reward.clone(),
            None => Rewards::default(),
        }
    }

    pub fn get_profile_complete_percentage(&self, account_id: AccountId) -> u32 {
        match self.profiles.get(&account_id) {
            Some(profile) => profile.get_profile_complete_percentage(),
            None => 0,
        }
    }
}
