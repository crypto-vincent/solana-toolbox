use std::collections::HashMap;

use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlService;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Prepare known accounts available on devnet
    let program_id = pubkey!("UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j");
    let program_data = pubkey!("9rtcXuviJngSZTRSCXxsHyd6qaWpqWSQ56SNumXAuLJ1");
    let mint_authority = pubkey!("7poxwHXi62Cwa57xdrpfoW2bUF7s8iXm1CU4jJqYPhu");
    let user = pubkey!("Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9");
    let collateral_mint =
        pubkey!("EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3");
    let user_collateral = ToolboxEndpoint::find_spl_associated_token_account(
        &user,
        &collateral_mint,
    );
    let name_record_header =
        pubkey!("8EodedXFv8DAJ6jGTg4DVXaBVJTVL3o4T2BWwTJTTJjw");
    let name_record_owner =
        pubkey!("8aU2gq8XgzNZr8z4noV87Sx8a3EV29gmi645qQERsaTD");
    // Prepare the IDL service
    let mut idl_service = ToolboxIdlService::new();
    // Check that we can resolve ATA with just the IDL
    let idl_program_ata = idl_service
        .get_or_resolve_program(
            &mut endpoint,
            &ToolboxEndpoint::SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
        )
        .await
        .unwrap()
        .unwrap();
    let create_ata_instruction_addresses = idl_service
        .resolve_instruction_addresses(
            &mut endpoint,
            idl_program_ata.instructions.get("create").unwrap(),
            &ToolboxEndpoint::SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
            &json!(null),
            &HashMap::from_iter([
                ("wallet".to_string(), user),
                ("mint".to_string(), collateral_mint),
            ]),
        )
        .await
        .unwrap();
    assert_eq!(
        *create_ata_instruction_addresses.get("ata").unwrap(),
        user_collateral,
    );
    // Check the state of a system account
    assert_account_info(
        &mut idl_service,
        &mut endpoint,
        &user,
        "system",
        "Wallet",
        json!(null),
    )
    .await;
    // Check the state of the collateral mint
    assert_account_info(
        &mut idl_service,
        &mut endpoint,
        &collateral_mint,
        "spl_token",
        "TokenMint",
        json!({
            "mint_authority": mint_authority.to_string(),
            "supply": 1000000000000000u64,
            "decimals": 9,
            "is_initialized": true,
            "freeze_authority": null,
        }),
    )
    .await;
    // Check the state of the collateral ATA
    assert_account_info(
        &mut idl_service,
        &mut endpoint,
        &user_collateral,
        "spl_token",
        "TokenAccount",
        json!({
            "mint": collateral_mint.to_string(),
            "owner": user.to_string(),
            "amount": 996906108000000u64,
            "delegate": null,
            "state": "Initialized",
            "is_native": null,
            "delegated_amount": 0,
            "close_authority": null,
        }),
    )
    .await;
    // Check the state of a known program
    assert_account_info(
        &mut idl_service,
        &mut endpoint,
        &program_id,
        "bpf_loader_upgradeable",
        "Program",
        json!({
            "program_data": program_data.to_string()
        }),
    )
    .await;
    // Check the state of a known program's executable data
    assert_account_info(
        &mut idl_service,
        &mut endpoint,
        &program_data,
        "bpf_loader_upgradeable",
        "ProgramData",
        json!({
            "slot": 347133692,
            "upgrade_authority": mint_authority.to_string(),
        }),
    )
    .await;
    // Check the state of a known name record header
    assert_account_info(
        &mut idl_service,
        &mut endpoint,
        &name_record_header,
        "spl_name_service",
        "NameRecordHeader",
        json!({
            "class": ToolboxEndpoint::SYSTEM_PROGRAM_ID.to_string(),
            "owner": name_record_owner.to_string(),
            "parent_name": ToolboxEndpoint::SYSTEM_PROGRAM_ID.to_string(),
        }),
    )
    .await;
}

async fn assert_account_info(
    idl_service: &mut ToolboxIdlService,
    endpoint: &mut ToolboxEndpoint,
    address: &Pubkey,
    program_name: &str,
    account_name: &str,
    account_state: Value,
) {
    let account_info = idl_service
        .get_and_infer_and_decode_account(endpoint, address)
        .await
        .unwrap();
    assert_eq!(
        account_info.program.metadata.name,
        Some(program_name.to_string())
    );
    assert_eq!(account_info.account.name, account_name);
    assert_eq!(account_info.state, account_state);
}
