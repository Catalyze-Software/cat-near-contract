use cat_near_contract::models::rewards::Rewards;
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
async fn test_check_profile_complete_reward() -> Result<(), Box<dyn std::error::Error>> {
    let (_, contract, account) = init().await?;

    // create incomplete profile
    let incomplete = account
        .call(contract.id(), "add_profile")
        .args_json(json!({"post_profile": {
            "username": "incomplete",
            "display_name": "incomplete",
            "first_name": "incomplete",
            "last_name": "incomplete",
            "extra": "incomplete"
        }}))
        .transact()
        .await?;

    let outcome_incomplete_profile_rewards: Rewards = account
        .view(contract.id(), "get_rewards")
        .args_json(json!({ "account_id": account.id()}))
        .await?
        .json()?;

    assert!(incomplete.is_success());
    assert!(!outcome_incomplete_profile_rewards.actions.profile_complete);
    assert!(outcome_incomplete_profile_rewards.points == 0);

    let complete = account
        .call(contract.id(), "edit_profile")
        .args_json(json!({"update_profile": {
            "username": "complete",
            "display_name": "complete",
            "first_name": "complete",
            "last_name": "complete",
            "extra": "complete",
            "email": "complete@complete.com",
            "country": "Zimbabwe",
            "about": "About completes",
            "profile_image": "https://example.com/image.jpg",
            "banner_image": "https://example.com/banner.jpg",
            "interests":[1,2,3],
        }}))
        .transact()
        .await?;

    let outcome_complete_profile_rewards: Rewards = account
        .view(contract.id(), "get_rewards")
        .args_json(json!({ "account_id": account.id()}))
        .await?
        .json()?;

    println!(
        "outcome_complete_profile_rewards: {:#?}",
        outcome_complete_profile_rewards
    );
    assert!(complete.is_success());
    assert!(outcome_complete_profile_rewards.actions.profile_complete);
    assert!(outcome_complete_profile_rewards.points == 100);
    Ok(())
}

#[tokio::test]
async fn test_join_group_reward() -> Result<(), Box<dyn std::error::Error>> {
    let (sandbox, contract, group_owner_account) = init().await?;

    let group_member_account = sandbox.dev_create_account().await?;

    // create group owner profile
    let group_owner_profile = group_owner_account
        .call(contract.id(), "add_profile")
        .args_json(json!({"post_profile": {
            "username": "owner",
            "display_name": "owner",
            "first_name": "owner",
            "last_name": "owner",
            "extra": "owner"
        }}))
        .transact()
        .await?;

    assert!(group_owner_profile.is_success());

    let group_member_profile = group_member_account
        .call(contract.id(), "add_profile")
        .args_json(json!({"post_profile": {
            "username": "member",
            "display_name": "member",
            "first_name": "member",
            "last_name": "member",
            "extra": "member"
        }}))
        .transact()
        .await?;

    assert!(group_member_profile.is_success());

    // create group
    let group_id = group_owner_account
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

    assert_eq!(group_id, 0, "Group ID should be equal to 0");

    // create group
    let group_2_id = group_owner_account
        .call(contract.id(), "add_group")
        .args_json(json!({
            "post_group": {
                "name": "Test Group 2",
                "description": "A test group 2",
                "website": "https://example2.com",
                "image": "ipfs://example2",
                "banner_image": "ipfs://example_banner2",
                "matrix_space_id": "space1232",
                "tags": vec![4,5,6]
            }
        }))
        .transact()
        .await
        .unwrap()
        .json::<u32>()
        .unwrap();

    assert_eq!(group_2_id, 1, "Group ID should be equal to 1");

    let joined_group = group_member_account
        .call(contract.id(), "join_group")
        .args_json(json!({
            "group_id": group_id
        }))
        .transact()
        .await?;

    assert!(joined_group.is_success());

    let member_rewards: Rewards = group_member_account
        .view(contract.id(), "get_rewards")
        .args_json(json!({ "account_id": group_member_account.id()}))
        .await?
        .json()?;

    assert!(member_rewards.points == 10);

    let joined_group_2 = group_member_account
        .call(contract.id(), "join_group")
        .args_json(json!({
            "group_id": group_2_id
        }))
        .transact()
        .await?;

    assert!(joined_group_2.is_success());

    let member_rewards: Rewards = group_member_account
        .view(contract.id(), "get_rewards")
        .args_json(json!({ "account_id": group_member_account.id()}))
        .await?
        .json()?;

    println!("member_rewards: {:#?}", member_rewards);
    assert!(member_rewards.points == 20);
    Ok(())
}
