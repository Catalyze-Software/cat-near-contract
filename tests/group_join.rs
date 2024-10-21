use cat_near_contract::models::{groups::GroupResponse, response_result::ResponseResult};
use near_sdk::serde_json::json;
use near_sdk::AccountId;
use near_workspaces::{network::Sandbox, Account, Contract, Worker};

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
    let group = user
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
        .json::<ResponseResult<GroupResponse>>()
        .unwrap();

    let new_user = sandbox.dev_create_account().await?;

    // First, create a profile for the user
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

    match group {
        ResponseResult::Err(_) => panic!("Group not found"),
        ResponseResult::Ok(group) => {
            // Join the group
            let result = new_user
                .call(contract.id(), "join_group")
                .args_json(json!({ "group_id": group.id }))
                .transact()
                .await;

            assert!(result.is_ok(), "Failed to join group: {:?}", result.err());

            // Verify user is in the group
            let is_in_group: bool = contract
                .view("is_user_in_group")
                .args_json(json!({
                    "account_id": new_user.id(),
                    "group_id": group.id
                }))
                .await?
                .json()?;

            assert!(is_in_group, "User should be in the group");
        }
    }

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
    let group = user
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
        .json::<ResponseResult<GroupResponse>>()
        .unwrap();

    match group {
        ResponseResult::Err(_) => panic!("Group not found"),
        ResponseResult::Ok(group) => {
            // Leave the group
            let result = user
                .call(contract.id(), "leave_group")
                .args_json(json!({ "group_id": group.id }))
                .transact()
                .await;

            println!("result: {:#?}", result);

            assert!(result.is_ok(), "Failed to leave group: {:?}", result.err());

            // Verify user is not in the group
            let is_in_group: bool = contract
                .view("is_user_in_group")
                .args_json(json!({
                    "account_id": user.id(),
                    "group_id": group.id
                }))
                .await?
                .json()?;

            println!("is_in_group: {:#?}", is_in_group);

            assert!(!is_in_group, "User should not be in the group");
        }
    }

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
        let group: ResponseResult<GroupResponse> = user
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
            .await
            .unwrap()
            .json::<ResponseResult<GroupResponse>>()
            .unwrap();

        if let ResponseResult::Ok(group) = group {
            group_ids.push(group.id);
        }
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
    let group = user
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
        .json::<ResponseResult<GroupResponse>>()
        .unwrap();

    match group {
        ResponseResult::Err(_) => panic!("Group not found"),
        ResponseResult::Ok(group) => {
            for user in &users {
                let _ = user
                    .call(contract.id(), "join_group")
                    .args_json(json!({ "group_id": group.id }))
                    .transact()
                    .await?;
            }

            let group_members: Vec<(AccountId, String)> = contract
                .view("get_group_members")
                .args_json(json!({ "group_id": group.id }))
                .await?
                .json()?;

            println!("Group members {:#?}", group_members);

            assert_eq!(group_members.len(), 4, "Group should have 4 members"); // three new users and one group creator as the owner
            for user in &users {
                assert!(
                    group_members.iter().any(|d| &d.0 == user.id()),
                    "User {} should be in the group",
                    user.id()
                );
                assert_eq!(
                    group_members.iter().find(|d| &d.0 == user.id()).unwrap().1,
                    "Member",
                    "User {} should have Member role",
                    user.id()
                );
            }
        }
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
    let group = user
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
        .json::<ResponseResult<GroupResponse>>()
        .unwrap();

    match group {
        ResponseResult::Err(_) => panic!("Group not found"),
        ResponseResult::Ok(group) => {
            // Get user role in group
            let user_role: Option<(AccountId, String)> = contract
                .view("get_user_role_in_group")
                .args_json(json!({
                    "account_id": user.id(),
                    "group_id": group.id
                }))
                .await?
                .json()?;

            assert_eq!(
                user_role,
                Some((user.id().clone(), "Owner".to_string())),
                "User should have Owner role in the group"
            );
        }
    }

    Ok(())
}
