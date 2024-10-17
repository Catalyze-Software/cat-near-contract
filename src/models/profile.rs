use crate::models::application_role::ApplicationRole;
use near_sdk::{env, near, AccountId};

#[derive(Clone, Default, Debug)]
#[near(serializers = ["json", "borsh"])]
pub struct Profile {
    pub username: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub about: String,
    pub email: String,
    pub date_of_birth: u64,
    pub city: String,
    pub state_or_province: String,
    pub country: String,
    pub profile_image: String, //url to the IPFS image
    pub banner_image: String,  //url to the IPFS image
    pub website: String,
    pub application_role: ApplicationRole,
    pub joined_groups: Vec<u32>,
    pub skills: Vec<u32>,
    pub interests: Vec<u32>,
    pub causes: Vec<u32>,
    pub extra: String,
    pub updated_on: u64,
    pub created_on: u64,
}

#[near(serializers = ["json"])]
#[derive(Clone, Debug)]
pub struct UpdateProfile {
    pub display_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub about: Option<String>,
    pub email: Option<String>,
    pub date_of_birth: Option<u64>,
    pub city: Option<String>,
    pub state_or_province: Option<String>,
    pub country: Option<String>,
    pub profile_image: Option<String>,
    pub banner_image: Option<String>,
    pub skills: Option<Vec<u32>>,
    pub interests: Option<Vec<u32>>,
    pub causes: Option<Vec<u32>>,
    pub website: Option<String>,
    pub extra: Option<String>,
}

impl Profile {
    pub fn add_group(&mut self, group_id: u32) {
        self.joined_groups.push(group_id);
    }

    pub fn remove_group(&mut self, group_id: u32) {
        self.joined_groups.retain(|&x| x != group_id);
    }

    pub fn is_group_member(&self, group_id: &u32) -> bool {
        self.joined_groups.contains(group_id)
    }

    pub fn get_group_ids(&self) -> Vec<u32> {
        self.joined_groups.clone()
    }
    //Check for None sent in UpdateProfile - Done
    pub fn update(&self, profile: UpdateProfile) -> Self {
        Self {
            username: self.username.clone(), // Assuming username can't be changed
            display_name: profile
                .display_name
                .unwrap_or_else(|| self.display_name.clone()),
            first_name: profile
                .first_name
                .unwrap_or_else(|| self.first_name.clone()),
            last_name: profile.last_name.unwrap_or_else(|| self.last_name.clone()),
            about: profile.about.unwrap_or_else(|| self.about.clone()),
            email: profile.email.unwrap_or_else(|| self.email.clone()),
            date_of_birth: profile.date_of_birth.unwrap_or(self.date_of_birth),
            city: profile.city.unwrap_or_else(|| self.city.clone()),
            state_or_province: profile
                .state_or_province
                .unwrap_or_else(|| self.state_or_province.clone()),
            country: profile.country.unwrap_or_else(|| self.country.clone()),
            profile_image: profile
                .profile_image
                .unwrap_or_else(|| self.profile_image.clone()),
            banner_image: profile
                .banner_image
                .unwrap_or_else(|| self.banner_image.clone()),
            application_role: self.application_role.clone(),
            joined_groups: self.joined_groups.clone(),
            skills: profile.skills.unwrap_or_else(|| self.skills.clone()),
            interests: profile.interests.unwrap_or_else(|| self.interests.clone()),
            causes: profile.causes.unwrap_or_else(|| self.causes.clone()),
            website: profile.website.unwrap_or_else(|| self.website.clone()),
            extra: profile.extra.unwrap_or_else(|| self.extra.clone()),
            updated_on: env::block_timestamp(),
            created_on: self.created_on,
        }
    }

    pub fn is_filled(&self) -> bool {
        let is_string_content_filled = vec![
            &self.first_name,
            &self.last_name,
            &self.email,
            &self.country,
            &self.about,
        ]
        .into_iter()
        .all(|s| !s.is_empty());

        let is_images_filled = !self.profile_image.is_empty() && !self.banner_image.is_empty();

        let is_interests_filled = self.interests.len() >= 3;

        is_string_content_filled && is_images_filled && is_interests_filled
    }

    pub fn get_profile_complete_percentage(&self) -> u32 {
        let mut percentage = 20;
        if !self.first_name.is_empty() {
            percentage += 10;
        }
        if !self.last_name.is_empty() {
            percentage += 10;
        }
        if !self.email.is_empty() {
            percentage += 10;
        }
        if !self.country.is_empty() {
            percentage += 10;
        }
        if !self.about.is_empty() {
            percentage += 10;
        }
        if !self.profile_image.is_empty() {
            percentage += 10;
        }
        if !self.banner_image.is_empty() {
            percentage += 10;
        }
        if self.interests.len() >= 3 {
            percentage += 10;
        }
        percentage
    }
}

impl From<PostProfile> for Profile {
    fn from(profile: PostProfile) -> Self {
        Self {
            username: profile.username,
            display_name: profile.display_name,
            application_role: ApplicationRole::default(),
            first_name: profile.first_name,
            last_name: profile.last_name,
            about: "".to_string(),
            email: "".to_string(),
            date_of_birth: 0,
            city: "".to_string(),
            state_or_province: "".to_string(),
            country: "".to_string(),
            profile_image: "".to_string(),
            banner_image: "".to_string(),
            joined_groups: vec![],
            skills: vec![],
            interests: vec![],
            causes: vec![],
            website: "".to_string(),
            extra: profile.extra,
            updated_on: env::block_timestamp(),
            created_on: env::block_timestamp(),
        }
    }
}

//users start with a default profile
//then use update to write more fields
//this reduces onboarding friction
#[near(serializers = ["json"])]
#[derive(Clone, Debug)]
pub struct PostProfile {
    pub username: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub extra: String, //make it optional or remove it
}

#[derive(Clone, Debug)]
#[near(serializers = ["json"])]
pub struct ProfileResponse {
    pub account_id: AccountId,
    pub username: String,
    pub display_name: String,
    pub application_role: ApplicationRole,
    pub first_name: String,
    pub last_name: String,
    pub about: String,
    pub email: String,
    pub date_of_birth: u64,
    pub city: String,
    pub state_or_province: String,
    pub country: String,
    pub profile_image: String,
    pub banner_image: String,
    pub skills: Vec<u32>,
    pub interests: Vec<u32>,
    pub causes: Vec<u32>,
    pub website: String,
    pub extra: String,
    pub updated_on: u64,
    pub created_on: u64,
}

impl ProfileResponse {
    pub fn new(account_id: AccountId, profile: Profile) -> Self {
        Self {
            account_id,
            username: profile.username,
            display_name: profile.display_name,
            about: profile.about,
            city: profile.city,
            country: profile.country,
            website: profile.website,
            skills: profile.skills,
            interests: profile.interests,
            causes: profile.causes,
            email: profile.email,
            application_role: profile.application_role,
            first_name: profile.first_name,
            last_name: profile.last_name,
            date_of_birth: profile.date_of_birth,
            state_or_province: profile.state_or_province,
            profile_image: profile.profile_image,
            banner_image: profile.banner_image,
            extra: profile.extra,
            updated_on: profile.updated_on,
            created_on: profile.created_on,
        }
    }
}
