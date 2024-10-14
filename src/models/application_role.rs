use core::fmt;
use near_sdk::near;

#[near(serializers=["json", "borsh"])]
#[derive(Default, Clone, Debug)]
pub enum ApplicationRole {
    Owner,
    #[default]
    Member,
}

impl fmt::Display for ApplicationRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ApplicationRole::*;
        match self {
            Owner => write!(f, "Owner"),
            Member => write!(f, "Member"),
        }
    }
}
