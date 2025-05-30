use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Prepare a payer
    let payer = Keypair::new();
    endpoint
        .request_airdrop(&payer.pubkey(), 1_000_000_000)
        .await
        .unwrap();
    // Generate a bunch of accounts we'll use to generate an history
    let mut users = vec![];
    for _ in 0..10 {
        users.push(Keypair::new());
    }
    // Generate a mint
    let collateral_mint_authority = Keypair::new();
    let collateral_mint = endpoint
        .process_spl_token_mint_new(
            &payer,
            &collateral_mint_authority.pubkey(),
            None,
            6,
        )
        .await
        .unwrap();
    // Mint some token to our users
    let mut users_collaterals = vec![];
    for (index, user) in users.iter().enumerate() {
        let user_collateral = endpoint
            .process_spl_associated_token_account_get_or_init(
                &payer,
                &user.pubkey(),
                &collateral_mint,
            )
            .await
            .unwrap();
        endpoint
            .process_spl_token_mint_to(
                &payer,
                &collateral_mint,
                &collateral_mint_authority,
                &user_collateral,
                1_000_000 + u64::try_from(index).unwrap(),
            )
            .await
            .unwrap();
        users_collaterals.push(user_collateral);
    }
    // Check that the token program contains all expected accounts
    let token_program_addresses = endpoint
        .search_addresses(&ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID, None, &[])
        .await
        .unwrap();
    assert_eq!(token_program_addresses.len(), 11);
    assert!(token_program_addresses.contains(&collateral_mint));
    for user_collateral in &users_collaterals {
        assert!(token_program_addresses.contains(user_collateral));
    }
    // Find token accounts by data size
    let token_accounts_addresses = endpoint
        .search_addresses(
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            Some(165),
            &[],
        )
        .await
        .unwrap();
    assert_eq!(token_accounts_addresses.len(), 10);
    for user_collateral in &users_collaterals {
        assert!(token_accounts_addresses.contains(user_collateral));
    }
    // Find token accounts by mint
    let token_accounts_addresses = endpoint
        .search_addresses(
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            None,
            &[(0, collateral_mint.as_ref())],
        )
        .await
        .unwrap();
    assert_eq!(token_accounts_addresses.len(), 10);
    for user_collateral in &users_collaterals {
        assert!(token_accounts_addresses.contains(user_collateral))
    }
    // Find mint by size and content
    let token_mints_addresses = endpoint
        .search_addresses(
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            Some(82),
            &[(4, collateral_mint_authority.pubkey().as_ref())],
        )
        .await
        .unwrap();
    assert_eq!(token_mints_addresses.len(), 1);
    assert!(token_mints_addresses.contains(&collateral_mint));
    // Find token account by owner
    for idx in 0..10 {
        let user = &users[idx];
        let user_addresses = endpoint
            .search_addresses(
                &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
                None,
                &[(0, collateral_mint.as_ref()), (32, user.pubkey().as_ref())],
            )
            .await
            .unwrap();
        assert_eq!(user_addresses.len(), 1);
        assert!(user_addresses.contains(&users_collaterals[idx]))
    }
    // Find token account by balance
    for (idx, user_collateral) in users_collaterals.iter().enumerate() {
        let amount = 1_000_000 + u64::try_from(idx).unwrap();
        let user_addresses = endpoint
            .search_addresses(
                &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
                Some(165),
                &[(64, &amount.to_le_bytes())],
            )
            .await
            .unwrap();
        assert_eq!(user_addresses.len(), 1);
        assert!(user_addresses.contains(user_collateral))
    }
}
