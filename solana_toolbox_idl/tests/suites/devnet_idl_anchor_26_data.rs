use std::fs::read_to_string;

use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlService;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Choosing our program_id
    let program_id = pubkey!("crdszSnZQu7j36KfsMJ4VEmMUTJgrNYXwoPVHUANpAu");
    // Parse and load IDL from file JSON directly (since it's not available onchain)
    let mut idl_service = ToolboxIdlService::new();
    idl_service.set_program(
        &program_id,
        Some(
            ToolboxIdlProgram::try_parse_from_str(
                &read_to_string("./tests/fixtures/idl_anchor_26.json").unwrap(),
            )
            .unwrap()
            .into(),
        ),
    );
    // Read the global market state content using the IDL
    let global_market_state =
        Pubkey::find_program_address(&[b"credix-marketplace"], &program_id).0;
    assert_account_info(
        &mut idl_service,
        &mut endpoint,
        &global_market_state,
        "credix",
        "GlobalMarketState",
        "seed",
        &json!("credix-marketplace"),
    )
    .await;
    // Read the program state content using the IDL
    let program_state =
        Pubkey::find_program_address(&[b"program-state"], &program_id).0;
    assert_account_info(
        &mut idl_service,
        &mut endpoint,
        &program_state,
        "credix",
        "ProgramState",
        "credix_multisig_key",
        &json!("Ej5zJzej7rrUoDngsJ3jcpfuvfVyWpcDcK7uv9cE2LdL"),
    )
    .await;
    // Read the market admins content using the IDL
    let market_admins = Pubkey::find_program_address(
        &[global_market_state.as_ref(), b"admins"],
        &program_id,
    )
    .0;
    assert_account_info(
        &mut idl_service,
        &mut endpoint,
        &market_admins,
        "credix",
        "MarketAdmins",
        "multisig",
        &json!("Ej5zJzej7rrUoDngsJ3jcpfuvfVyWpcDcK7uv9cE2LdL"),
    )
    .await;
}

async fn assert_account_info(
    idl_service: &mut ToolboxIdlService,
    endpoint: &mut ToolboxEndpoint,
    address: &Pubkey,
    program_name: &str,
    account_name: &str,
    account_state_key: &str,
    account_state_value: &Value,
) {
    let account_info = idl_service
        .get_and_infer_and_decode_account(endpoint, address)
        .await
        .unwrap();
    assert_eq!(
        account_info.program.metadata.name,
        Some(program_name.to_string()),
    );
    assert_eq!(account_info.account.name, account_name);
    assert_eq!(&account_info.state[account_state_key], account_state_value);
}
