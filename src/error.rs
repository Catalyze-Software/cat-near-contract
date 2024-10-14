use near_sdk::near;

#[derive(Clone, Debug)]
#[near(serializers = ["json", "borsh"])]
pub enum GroupError {
    ProfileNotFound,
    GroupNotFound,
    AlreadyMember,
    NotMember,
    UserAlreadyInGroup,
}

impl AsRef<str> for GroupError {
    fn as_ref(&self) -> &str {
        match self {
            GroupError::ProfileNotFound => "Profile not found",
            GroupError::GroupNotFound => "Group not found",
            GroupError::AlreadyMember => "Already a member of this group",
            GroupError::NotMember => "Not a member of this group",
            GroupError::UserAlreadyInGroup => "User already in group",
        }
    }
}
