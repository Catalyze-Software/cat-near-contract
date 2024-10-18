use near_sdk::serde_json::json;
use near_sdk::AccountId;
use near_workspaces::{network::Sandbox, Account, Contract, Worker};
use std::collections::HashMap;

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
async fn test_join_group() -> Result<(), Box<dyn std::error::Error>> {
    let (sandbox, contract, user) = init().await?;

    // First, create a profile for the user
    // CREATED A PROFILE (INSERT)
    let _ = user
        .call(contract.id(), "add_profile")
        .args_json(json!({"post_profile": {
            "username": "jassification",
            "display_name": "Jas",
            "first_name": "Jaswinder",
            "last_name": "Singh",
            "extra": "Test"
        }}))
        .transact()
        .await?;

    
    // Create a group
    // CREATE A GROUP WHERE THE USER IS ADDED AS A MEMBER UPON INSERTION BY
    ///```
    /// members: Members::new_with_owner(env::signer_account_id()),
    ///```
    /// PROFILE IS NEVER UPDATED WITH TNE NEW GROUP ID 
    let group_id: u32 = user
        .call(contract.id(), "add_group")
        .args_json(json!({
            "post_group": {
                "name": "Test Group",
                "description": "A test group",
                "website": "https://example.com",
                "image": "ipfs://example",
                "banner_image": "ipfs://example_banner",
                "matrix_space_id": "space123",
                "tags": vec![1, 2, 3]
            }
        }))
        .transact()
        .await
        .unwrap()
        .json::<u32>()
        .unwrap();

    let new_user = sandbox.dev_create_account().await?;

    let _ = new_user
        .call(contract.id(), "add_profile")
        .args_json(json!({"post_profile": {
            "username": "testname",
            "display_name": "Test",
            "first_name": "Test",
            "last_name": "Test",
            "extra": "Test"
        }}))
        .transact()
        .await?;

    // ONLY INSERTS THE USER IN THE GROUP (INSERT)
    // DOES NOT INSERT THE GROUP ID IN THE USER
    let result = new_user
        .call(contract.id(), "join_group")
        .args_json(json!({ "group_id": group_id }))
        .transact()
        .await;

    assert!(result.is_ok(), "Failed to join group: {:?}", result.err());

    // ONLY CHECKS IF THE USER IS IN THE GROUP
    // NOT IF THE GROUP IS ADDED TO THE USER PROFILE
    let is_in_group: bool = contract
        .view("is_user_in_group")
        .args_json(json!({
            "account_id": new_user.id(),
            "group_id": group_id
        }))
        .await?
        .json()?;

    assert!(is_in_group, "User should be in the group");

    Ok(())
}

#[tokio::test]
async fn test_leave_group() -> Result<(), Box<dyn std::error::Error>> {
    let (_, contract, user) = init().await?;

    // First, create a profile for the user
    let _ = user
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

    // Create a group
    let group_id: u32 = user
        .call(contract.id(), "add_group")
        .args_json(json!({
            "post_group": {
                "name": "Test Group",
                "description": "A test group",
                "website": "https://example.com",
                "image": "ipfs://example",
                "banner_image": "ipfs://example_banner",
                "matrix_space_id": "space123",
                "tags": vec![1, 2, 3]
            }
        }))
        .transact()
        .await
        .unwrap()
        .json::<u32>()
        .unwrap();

    // Leave the group
    let result = user
        .call(contract.id(), "leave_group")
        .args_json(json!({ "group_id": group_id }))
        .transact()
        .await;

    println!("result: {:#?}", result);

    assert!(result.is_ok(), "Failed to leave group: {:?}", result.err());

    // Verify user is not in the group
    let is_in_group: bool = contract
        .view("is_user_in_group")
        .args_json(json!({
            "account_id": user.id(),
            "group_id": group_id
        }))
        .await?
        .json()?;

    println!("is_in_group: {:#?}", is_in_group);

    assert!(!is_in_group, "User should not be in the group");

    Ok(())
}

#[tokio::test]
async fn test_get_user_groups() -> Result<(), Box<dyn std::error::Error>> {
    let (_, contract, user) = init().await?;

    // Setup: Create profile, create multiple groups, join groups
    // First, create a profile for the user
    let _ = user
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

    let mut group_ids = vec![];
    for i in 0..3 {
        let group_id: u32 = user
            .call(contract.id(), "add_group")
            .args_json(json!({
                    "post_group": {
                    "name": format!("Test Group {}", i),
                    "description": "A test group",
                    "website": "https://example.com",
                    "image": "ipfs://example",
                    "banner_image": "ipfs://example_banner",
                    "matrix_space_id": format!("space{}", i),
                    "tags": vec![1, 2, 3]
                }
            }
                ))
            .transact()
            .await?
            .json()?;
        group_ids.push(group_id);
    }

    // Get user groups
    // No need to join as the group creator gets added as owner already
    let user_groups: Vec<u32> = contract
        .view("get_user_groups")
        .args_json(json!({ "account_id": user.id() }))
        .await?
        .json()?;

    assert_eq!(user_groups.len(), 3, "User should be in 3 groups");
    assert_eq!(
        user_groups, group_ids,
        "User groups should match created groups"
    );

    Ok(())
}

#[tokio::test]
async fn test_get_group_members() -> Result<(), Box<dyn std::error::Error>> {
    let (sandbox, contract, user) = init().await?;

    // First, create a profile for the user otherwise, it can't create a group
    let _ = user
        .call(contract.id(), "add_profile")
        .args_json(json!({"post_profile": {
            "username": "jassification",
            "display_name": "Jas",
            "first_name": "Jaswinder",
            "last_name": "Singh",
            "extra": "Test"
        }}))
        .transact()
        .await?;

    // Setup: Create profiles, create group, users join group
    let mut users = vec![];
    for i in 0..3 {
        let new_user = sandbox.dev_create_account().await?;
        let _ = new_user
            .call(contract.id(), "add_profile")
            .args_json(json!({"post_profile": {
                "username": format!("testuser{}", i),
                "display_name": format!("Test User {}", i),
                "first_name": "Jaswinder",
                "last_name": "Singh",
                "extra": "extra"
            }}))
            .transact()
            .await?;
        users.push(new_user);
    }

    // Create a group
    let group_id: u32 = user
        .call(contract.id(), "add_group")
        .args_json(json!({
            "post_group": {
                "name": "Test Group",
                "description": "A test group",
                "website": "https://example.com",
                "image": "ipfs://example",
                "banner_image": "ipfs://example_banner",
                "matrix_space_id": "space123",
                "tags": vec![1, 2, 3]
            }
        }))
        .transact()
        .await
        .unwrap()
        .json::<u32>()
        .unwrap();

    for user in &users {
        let _ = user
            .call(contract.id(), "join_group")
            .args_json(json!({ "group_id": group_id }))
            .transact()
            .await?;
    }

    // Get group members
    let group_members: HashMap<AccountId, String> = contract
        .view("get_group_members")
        .args_json(json!({ "group_id": group_id }))
        .await?
        .json()?;

    println!("Group members {:#?}", group_members);

    assert_eq!(group_members.len(), 4, "Group should have 4 members"); // three new users and one group creator as the owner
    for user in &users {
        assert!(
            group_members.contains_key(user.id()),
            "User {} should be in the group",
            user.id()
        );
        assert_eq!(
            group_members.get(user.id()).unwrap(),
            "Member",
            "User {} should have Member role",
            user.id()
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_get_user_role_in_group() -> Result<(), Box<dyn std::error::Error>> {
    let (_, contract, user) = init().await?;

    // Setup: Create profile, create group, join group
    // First, create a profile for the user otherwise, it can't create a group
    let _ = user
        .call(contract.id(), "add_profile")
        .args_json(json!({"post_profile": {
            "username": "jassification",
            "display_name": "Jas",
            "first_name": "Jaswinder",
            "last_name": "Singh",
            "extra": "Test"
        }}))
        .transact()
        .await?;

    // Create a group
    let group_id: u32 = user
        .call(contract.id(), "add_group")
        .args_json(json!({
            "post_group": {
                "name": "Test Group",
                "description": "A test group",
                "website": "https://example.com",
                "image": "ipfs://example",
                "banner_image": "ipfs://example_banner",
                "matrix_space_id": "space123",
                "tags": vec![1, 2, 3]
            }
        }))
        .transact()
        .await
        .unwrap()
        .json::<u32>()
        .unwrap();

    // Get user role in group
    let user_role: Option<String> = contract
        .view("get_user_role_in_group")
        .args_json(json!({
            "account_id": user.id(),
            "group_id": group_id
        }))
        .await?
        .json()?;

    assert_eq!(
        user_role,
        Some("Owner".to_string()),
        "User should have Owner role in the group"
    );

    Ok(())
}
