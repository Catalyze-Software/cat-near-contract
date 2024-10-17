use near_sdk::near;

#[derive(Clone, Debug)]
#[near(serializers = ["json", "borsh"])]
pub enum GenericError {
    ProfileNotFound,
    ProfileAlreadyExists,
    GroupNotFound,
    GroupNotAdded,
    AlreadyMember,
    NotMember,
    UserAlreadyInGroup,
}

impl AsRef<str> for GenericError {
    fn as_ref(&self) -> &str {
        match self {
            GenericError::ProfileNotFound => "Profile not found",
            GenericError::ProfileAlreadyExists => "Profile already exists",
            GenericError::GroupNotFound => "Group not found",
            GenericError::AlreadyMember => "Already a member of this group",
            GenericError::NotMember => "Not a member of this group",
            GenericError::UserAlreadyInGroup => "User already in group",
            GenericError::GroupNotAdded => "Group not added",
        }
    }
}
