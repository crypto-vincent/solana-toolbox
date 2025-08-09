use std::collections::HashMap;

use solana_sdk::account::Account;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Define dummy preloaded accounts
    let preloaded_account_address = Keypair::new().pubkey();
    let preloaded_account_lamports = 1_000_000_000;
    let preloaded_account_data = Keypair::new().to_bytes();
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test_with_config(
        &[],
        HashMap::default(),
        HashMap::from_iter(vec![(
            preloaded_account_address,
            Account {
                lamports: preloaded_account_lamports,
                data: preloaded_account_data.to_vec(),
                ..Default::default()
            },
        )]),
    )
    .await;
    // Assert the account content from the endpoint
    assert_eq!(
        endpoint
            .get_balance(&preloaded_account_address)
            .await
            .unwrap(),
        preloaded_account_lamports
    );
    assert_eq!(
        endpoint
            .get_account(&preloaded_account_address)
            .await
            .unwrap()
            .unwrap()
            .data,
        preloaded_account_data.to_vec()
    );
}
