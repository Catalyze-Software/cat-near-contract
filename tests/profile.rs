use cat_near_contract::models::{profile::ProfileResponse, response_result::ResponseResult};
use near_workspaces::{network::Sandbox, Account, Contract, Worker};
use serde_json::json;

async fn init() -> Result<(Worker<Sandbox>, Contract, Account), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;
    let contract = sandbox.dev_deploy(&contract_wasm).await?;
    let user_account = sandbox.dev_create_account().await?;

    let outcome = user_account
        .call(contract.id(), "new")
        .args_json(json!({}))
        .transact()
        .await?;
    assert!(outcome.is_success());

    Ok((sandbox, contract, user_account))
}

#[tokio::test]
async fn test_add_profile() -> Result<(), Box<dyn std::error::Error>> {
    let (_, contract, user_account) = init().await?;

    let outcome_post_profile = user_account
        .call(contract.id(), "add_profile")
        .args_json(json!({"post_profile": {
            "username": "jassification",
            "display_name": "Jas",
            "first_name": "Jaswinder",
            "last_name": "Singh",
            "extra": "extra"
        }}))
        .transact()
        .await?;

    assert!(outcome_post_profile.is_success());
    Ok(())
}

#[tokio::test]
async fn test_edit_profile() -> Result<(), Box<dyn std::error::Error>> {
    let (_, contract, user_account) = init().await?;

    // First, add a profile
    let _ = user_account
        .call(contract.id(), "add_profile")
        .args_json(json!({"post_profile": {
            "username": "jassification",
            "display_name": "Jas",
            "first_name": "Jaswinder",
            "last_name": "Singh",
            "extra": "extra"
        }}))
        .transact()
        .await?;

    let outcome_update_profile = user_account
        .call(contract.id(), "edit_profile")
        .args_json(json!({"update_profile": {
            "display_name": "Jassi",
            "first_name": "Jas",
            "last_name": "Singh",
            "about": "About",
            "date_of_birth": 123456,
            "extra": "extra",
            "city":"Mumbai",
            "state_or_province":"Maharashtra",
            "country":"India",
            "profile_image":"profile_image",
            "skills":[1,2,3],
            "interests":[1,2,3],
            "causes":[1,2,3],
            "website":"website"
        }}))
        .transact()
        .await?;

    assert!(outcome_update_profile.is_success());

    Ok(())
}

#[tokio::test]
async fn test_get_profile() -> Result<(), Box<dyn std::error::Error>> {
    let (_, contract, user_account) = init().await?;

    // First, add a profile
    let _ = user_account
        .call(contract.id(), "add_profile")
        .args_json(json!({"post_profile": {
            "username": "jassification",
            "display_name": "Jas",
            "first_name": "Jaswinder",
            "last_name": "Singh",
            "extra": "extra"
        }}))
        .transact()
        .await?;

    let outcome_get_profile: ResponseResult<ProfileResponse> = user_account
        .view(contract.id(), "get_profile")
        .args_json(json!({ "account_id": user_account.id()}))
        .await?
        .json()?;

    match outcome_get_profile {
        ResponseResult::Ok(outcome_get_profile) => {
            assert_eq!(outcome_get_profile.username, "jassification");
            assert_eq!(outcome_get_profile.display_name, "Jas");
            assert_eq!(outcome_get_profile.first_name, "Jaswinder");
            assert_eq!(outcome_get_profile.last_name, "Singh");
            println!("outcome_get_profile: {:#?}", outcome_get_profile);
        }
        ResponseResult::Err(_) => panic!("Profile not found"),
    }

    Ok(())
}

#[tokio::test]
async fn test_get_profiles() -> Result<(), Box<dyn std::error::Error>> {
    let (sandbox, contract, user_account1) = init().await?;

    // Create a second user account
    let user_account2 = sandbox.dev_create_account().await?;

    // Add profiles for both users
    for user in [&user_account1, &user_account2] {
        let _ = user
            .call(contract.id(), "add_profile")
            .args_json(json!({"post_profile": {
                "username": format!("user_{}", user.id()),
                "display_name": "Test User",
                "first_name": "Test",
                "last_name": "User",
                "extra": "extra"
            }}))
            .transact()
            .await?;
    }

    // Test get_profiles function
    let outcome_get_profiles: Vec<ProfileResponse> = user_account1
        .view(contract.id(), "get_profiles")
        .args_json(json!({
            "account_ids": [user_account1.id(), user_account2.id()]
        }))
        .await?
        .json()?;

    println!("outcome_get_profiles: {:#?}", outcome_get_profiles);
    assert_eq!(outcome_get_profiles.len(), 2);
    assert_eq!(
        outcome_get_profiles[0].username,
        format!("user_{}", user_account1.id())
    );
    assert_eq!(
        outcome_get_profiles[1].username,
        format!("user_{}", user_account2.id())
    );
    Ok(())
}

// #[tokio::test]
// async fn test_edit_profile_no_insert() -> Result<(), Box<dyn std::error::Error>> {
//     let (_, contract, user_account) = init().await?;

//     let outcome_post_profile = user_account
//         .call(contract.id(), "add_profile")
//         .args_json(json!({"post_profile": {
//             "username": "new",
//             "display_name": "new",
//             "first_name": "new",
//             "last_name": "new",
//             "extra": "new"
//         }}))
//         .transact()
//         .await?;

//     assert!(outcome_post_profile.is_success());

//     let outcome_edit_profile = user_account
//         .call(contract.id(), "edit_profile_no_insert")
//         .args_json(json!({"update_profile": {
//             "display_name": "updated",
//             "first_name": "updated",
//             "last_name": "updated",
//         }}))
//         .transact()
//         .await?;

//     assert!(outcome_edit_profile.is_success());

//     let outcome_get_profile: ResponseResult<ProfileResponse> = user_account
//         .view(contract.id(), "get_profile")
//         .args_json(json!({ "account_id": user_account.id()}))
//         .await?
//         .json()?;

//     println!("outcome_get_profile: {:#?}", outcome_get_profile);
//     match outcome_get_profile {
//         ResponseResult::Ok(outcome_get_profile) => {
//             assert_eq!(outcome_get_profile.display_name, "updated");
//             assert_eq!(outcome_get_profile.first_name, "updated");
//             assert_eq!(outcome_get_profile.last_name, "updated");
//         }
//         ResponseResult::Err(_) => panic!("Profile not found"),
//     }
//     Ok(())
// }

// #[tokio::test]
// async fn test_edit_profile_with_insert() -> Result<(), Box<dyn std::error::Error>> {
//     let (_, contract, user_account) = init().await?;

//     let outcome_post_profile = user_account
//         .call(contract.id(), "add_profile")
//         .args_json(json!({"post_profile": {
//             "username": "new",
//             "display_name": "new",
//             "first_name": "new",
//             "last_name": "new",
//             "extra": "new"
//         }}))
//         .transact()
//         .await?;

//     assert!(outcome_post_profile.is_success());

//     let outcome_post_profile = user_account
//         .call(contract.id(), "edit_profile_with_insert")
//         .args_json(json!({"update_profile": {
//             "display_name": "updated",
//             "first_name": "updated",
//             "last_name": "updated",
//         }}))
//         .transact()
//         .await?;

//     assert!(outcome_post_profile.is_success());
//     Ok(())
// }
