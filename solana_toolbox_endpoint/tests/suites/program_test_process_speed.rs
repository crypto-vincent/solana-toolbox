use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Make a payer
    let payer = Keypair::new();
    endpoint
        .request_airdrop(&payer.pubkey(), 1_000_000_000)
        .await
        .unwrap();
    // Create many users
    let mut users = vec![];
    for _ in 0..500 {
        users.push(Keypair::new());
    }
    // Send lamports to all users
    for user in &users {
        endpoint
            .process_system_transfer(&payer, &payer, &user.pubkey(), 1_000_000)
            .await
            .unwrap();
    }
    // Have all the users send some back
    for user in &users {
        endpoint
            .process_system_transfer(&user, &user, &payer.pubkey(), 100_000)
            .await
            .unwrap();
    }
}
