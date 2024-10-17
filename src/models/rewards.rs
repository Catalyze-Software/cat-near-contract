use near_sdk::{env, near};

#[derive(Clone, Debug)]
#[near(serializers = ["json", "borsh"])]
pub struct Rewards {
    pub actions: RewardActions,
    pub points: u32,
    pub updated_on: u64,
    pub created_on: u64,
}

#[derive(Clone, Debug, Default)]
#[near(serializers = ["json", "borsh"])]
pub struct RewardActions {
    pub profile_complete: bool,
    pub group_join_action_history: Vec<u32>,
}

impl Default for Rewards {
    fn default() -> Self {
        Self {
            actions: Default::default(),
            points: 0,
            updated_on: env::block_timestamp(),
            created_on: env::block_timestamp(),
        }
    }
}

impl Rewards {
    pub fn profile_complete(&mut self) -> Self {
        if !self.actions.profile_complete {
            self.actions.profile_complete = true;
            self.points += 100;
            self.updated_on = env::block_timestamp();
        }
        self.clone()
    }

    pub fn group_join(&mut self, group_id: u32) -> Self {
        if !self.actions.group_join_action_history.contains(&group_id) {
            self.points += 10;
            self.actions.group_join_action_history.push(group_id);
            self.updated_on = env::block_timestamp();
        }
        self.clone()
    }
}
