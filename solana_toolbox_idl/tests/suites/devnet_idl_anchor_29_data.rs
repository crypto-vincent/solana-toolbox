use serde_json::json;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlService;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // The devnet program we'll lookup
    let program_id = pubkey!("Ee5CDFHQmdUQMEnM3dJZMiLaBuP2Wr8WBVYM7UZPPb6E");
    // Important account addresses
    let realm_pda = Pubkey::find_program_address(&[b"realm"], &program_id);
    let realm = realm_pda.0;
    let realm_bump = realm_pda.1;
    let uct_mint_pda = Pubkey::find_program_address(
        &[b"uct_mint", &realm.to_bytes()],
        &program_id,
    );
    let uct_mint = uct_mint_pda.0;
    let uct_mint_bump = uct_mint_pda.1;
    // Actually fetch our account using the auto-resolved IDL on-chain
    let realm_info = ToolboxIdlService::new()
        .get_and_infer_and_decode_account(&mut endpoint, &realm)
        .await
        .unwrap();
    // Check that the account was parsed properly and values matches
    assert_eq!(
        realm_info.program.metadata.name,
        Some("redemption".to_string()),
    );
    assert_eq!(realm_info.account.name, "Realm");
    assert_eq!(&realm_info.state["bump"], &json!(realm_bump));
    // Related "USDC mint" account checks
    assert_eq!(
        &realm_info.state["usdc_mint"],
        &json!("H7JmSvR6w6Qrp9wEbw4xGEBkbh95Jc9C4yXYYYvWmF8B"),
    );
    // Related "UCT mint" account checks
    assert_eq!(&realm_info.state["uct_mint_bump"], &json!(uct_mint_bump));
    assert_eq!(&realm_info.state["uct_mint"], &json!(uct_mint.to_string()));
}
