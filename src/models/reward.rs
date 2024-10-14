use near_sdk::near;

#[derive(Clone, Debug)]
#[near(serializers = ["json", "borsh"])]
pub struct Rewards {
    pub action: RewardAction,
    pub points: u32,
    pub updated_on: u64,
    pub created_on: u64,
}

#[derive(Clone, Debug)]
#[near(serializers = ["json", "borsh"])]
pub enum RewardAction {
    ProfileComplete(u32),
    GroupJoin(u32),
}
