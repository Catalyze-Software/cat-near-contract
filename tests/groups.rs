use cat_near_contract::models::groups::GroupResponse;
use near_sdk::serde_json::json;
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
async fn test_add_group() -> Result<(), Box<dyn std::error::Error>> {
    let (_, contract, user) = init().await?;

    let result = user
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

    assert_eq!(result, 0, "Group ID should be equal to 0");
    Ok(())
}

#[tokio::test]
async fn test_edit_group() -> Result<(), Box<dyn std::error::Error>> {
    let (_, contract, user) = init().await?;

    // First, add a group
    let group_id = user
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

    // Now, edit the group
    let result = user
        .call(contract.id(), "edit_group")
        .args_json(json!({
            "id": group_id,
            "update_group": {
                "name": "Updated Test Group",
                "description": "An updated test group"
            }
        }))
        .transact()
        .await;

    assert!(result.is_ok(), "Edit group should succeed");

    // Verify the update
    let updated_group = contract
        .view("get_group")
        .args_json(json!({"id": group_id}))
        .await
        .unwrap()
        .json::<Option<GroupResponse>>()
        .unwrap();

    assert_eq!(updated_group.unwrap().name, "Updated Test Group");
    Ok(())
}

#[tokio::test]
async fn test_get_group_by_name() -> Result<(), Box<dyn std::error::Error>> {
    let (_, contract, user) = init().await?;

    // Add a group
    let _ = user
        .call(contract.id(), "add_group")
        .args_json(json!({
            "post_group": {
                "name": "Unique Test Group",
                "description": "A unique test group",
                "website": "https://example.com",
                "image": "ipfs://example",
                "banner_image": "ipfs://example_banner",
                "matrix_space_id": "space123",
                "tags": vec![1, 2, 3]
            }
        }))
        .transact()
        .await
        .unwrap();

    // Get the group by name
    let result = contract
        .view("get_group_by_name")
        .args_json(json!({"name": "Unique Test Group"}))
        .await
        .unwrap()
        .json::<Option<GroupResponse>>()
        .unwrap();

    assert!(result.is_some(), "Group should be found");
    assert_eq!(result.unwrap().name, "Unique Test Group");

    Ok(())
}

#[tokio::test]
async fn test_get_groups() -> Result<(), Box<dyn std::error::Error>> {
    let (_, contract, user) = init().await?;

    // Add multiple groups
    for i in 0..4 {
        let _ = user
            .call(contract.id(), "add_group")
            .args_json(json!({
                "post_group": {
                    "name": format!("Test Group {}", i),
                    "description": format!("Test group {}", i),
                    "website": "https://example.com",
                    "image": "ipfs://example",
                    "banner_image": "ipfs://example_banner",
                    "matrix_space_id": format!("space{}", i),
                    "tags": vec![1, 2, 3]
                }
            }))
            .transact()
            .await
            .unwrap();
    }

    // Get groups with pagination
    let result = contract
        .view("get_groups")
        .args_json(json!({"index": 1, "limit": 4}))
        .await
        .unwrap()
        .json::<Vec<GroupResponse>>()
        .unwrap();

    assert_eq!(result.len(), 3, "Should return 3 groups");

    Ok(())
}

#[tokio::test]
async fn test_get_groups_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let (_, contract, user) = init().await?;

    // Add multiple groups and collect their IDs
    let mut group_ids = Vec::new();
    for i in 0..3 {
        let group_id = user
            .call(contract.id(), "add_group")
            .args_json(json!({
                "post_group": {
                    "name": format!("Test Group {}", i),
                    "description": format!("Test group {}", i),
                    "website": "https://example.com",
                    "image": "ipfs://example",
                    "banner_image": "ipfs://example_banner",
                    "matrix_space_id": format!("space{}", i),
                    "tags": vec![1, 2, 3]
                }
            }))
            .transact()
            .await
            .unwrap()
            .json::<u32>()
            .unwrap();
        group_ids.push(group_id);
    }

    // Get groups by ID
    let result = contract
        .view("get_groups_by_id")
        .args_json(json!({"ids": group_ids}))
        .await
        .unwrap()
        .json::<Vec<GroupResponse>>()
        .unwrap();

    assert_eq!(result.len(), 3, "Should return 3 groups");
    assert_eq!(result[0].id, group_ids[0], "First group ID should match");

    Ok(())
}
