use near_sdk::near;

#[derive(Clone, Debug)]
#[near(serializers = ["json", "borsh"])]
pub enum Error {
    ProfileNotFound,
    GroupNotFound,
    AlreadyMember,
    NotMember,
    UserAlreadyInGroup,
}

impl AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        match self {
            Error::ProfileNotFound => "Profile not found",
            Error::GroupNotFound => "Group not found",
            Error::AlreadyMember => "Already a member of this group",
            Error::NotMember => "Not a member of this group",
            Error::UserAlreadyInGroup => "User already in group",
        }
    }
}
