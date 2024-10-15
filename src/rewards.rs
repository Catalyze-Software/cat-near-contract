use crate::{models::rewards::Rewards, Contract, ContractExt};
use near_sdk::{env, near};

//Nice to have
//add validation to PostProfile and UpdateProfile

#[near]
impl Contract {
    pub fn get_rewards(&self) -> Option<Rewards> {
        let account_id = env::signer_account_id();
        Some(self.rewards.get(&account_id)?.clone())
    }
}
