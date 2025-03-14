use std::fs::read_to_string;

use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Parse IDL from file JSON directly
    let idl_string =
        read_to_string("./tests/fixtures/idl_anchor_0_30.json").unwrap();
    let idl_program = ToolboxIdlProgram::try_parse_from_str(&idl_string).unwrap();
    // Prepare instruction args
    let mut instruction_args_metadata_bytes = vec![];
    for index in 0..512 {
        instruction_args_metadata_bytes.push(Value::from(index % 100));
    }
    let instruction_args_value = json!({
        "params": {
            "index": 42,
            "funding_goal_collateral_amount": 41,
            "funding_phase_duration_seconds": 99,
            "metadata": {
                "length": 22,
                "bytes": instruction_args_metadata_bytes,
            },
        },
    });
    // Compile / decompile the instruction args and check that they match the original
    assert_eq!(
        &instruction_args_value,
        &idl.decompile_instruction_data(
            "campaign_create",
            &idl.compile_instruction_data(
                "campaign_create",
                &instruction_args_value
            )
            .unwrap()
        )
        .unwrap()
    );
    // Prepare an account contents
    let mut account_metadata_bytes = vec![];
    for index in 0..512 {
        account_metadata_bytes.push(Value::from(index % 100));
    }
    let account = ToolboxIdlAccount {
        name: "Campaign".to_string(),
        state: json!({
            "index": 77,
            "bump": 99,
            "authority": Pubkey::new_unique().to_string(),
            "collateral_mint": Pubkey::new_unique().to_string(),
            "redeemable_mint": Pubkey::new_unique().to_string(),
            "funding_goal_collateral_amount": 11,
            "total_deposited_collateral_amount": 22,
            "total_claimed_redeemable_amount": 33,
            "funding_phase_start_unix_timestamp": -44,
            "funding_phase_end_unix_timestamp": -55,
            "extracted_collateral_amount": 66,
            "metadata": {
                "length": 99,
                "bytes": account_metadata_bytes,
            }
        }),
    };
    // Decompile the account content and check that it matches the original
    assert_eq!(
        account,
        idl.decompile_account(&idl.compile_account(&account).unwrap())
            .unwrap()
    );
    // Prepare an account contents
    let account = ToolboxIdlAccount {
        name: "Pledge".to_string(),
        state: json!({
            "bump": 44,
            "deposited_collateral_amount": 999,
            "claimed_redeemable_amount": 22,
        }),
    };
    // Decompile the account content and check that it matches the original
    assert_eq!(
        account,
        idl.decompile_account(&idl.compile_account(&account).unwrap())
            .unwrap()
    );
}
