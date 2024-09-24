use serde_json::json;
use serde_json::Value;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
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

    let outcome_post_profile = user_account
        .call(contract.id(), "add_profile")
        .args_json(json!({"post_profile": {
            "username": "jassification",
            "display_name": "Jas",
            "first_name": "Jaswinder",
            "last_name": "Singh",
            "extra": "extra"
        }
        }))
        .transact()
        .await?;
    //println!("outcome_post_profile: {:?}", outcome_post_profile);
    assert!(outcome_post_profile.is_success());

    let outcome_update_profile = user_account
        .call(contract.id(), "edit_profile")
        .args_json(json!({"update_profile": {
            "display_name": "Jassi",
            "first_name": "Jas",
            "last_name": "Singh",
            "about": "About",
            //"email": "email",
            "date_of_birth": 123456,
            "extra": "extra",
            "city":"Mumbai",
            "state_or_province":"Maharashtra",
            "country":"India",
            "profile_image":"profile_image",
            //"banner_image":"banner_image",
            "skills":[1,2,3],
            "interests":[1,2,3],
            "causes":[1,2,3],
            "website":"website"
        }
        }))
        .transact()
        .await?;
    //println!("outcome_update_profile: {:?}", outcome_update_profile);
    assert!(outcome_update_profile.is_success());

    let outcome_get_profile = user_account
        .view(contract.id(), "get_profile")
        .args_json(json!({ "account_id": user_account.id()}))
        .await?;

    println!(
        "outcome_get_profile: {:#?}",
        outcome_get_profile.json::<Value>()
    );

    Ok(())
}
