use std::str::FromStr;
use std::time::Duration;
use std::time::SystemTime;

use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::instruction::InstructionError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::TransactionError;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointExecution;
use solana_toolbox_endpoint::ToolboxEndpointExecutionStep;
use solana_toolbox_endpoint::ToolboxEndpointExecutionStepCall;

#[tokio::test]
pub async fn run() {
    // Create the endpoint pointing to devnet
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Lookup a transaction execution that already happened and succeeded
    let signature_success = Signature::from_str("2pqW2HvC2FqVr1GkSgLrPCp55THBzYWP6oMkaB6bZzaRXKYNJ2wfcBCu3M9r64SVcX3fEC5EomwxF939kn4pYXBW").unwrap();
    let execution_success =
        endpoint.get_execution(&signature_success).await.unwrap();
    // Check that the execution details are correct
    assert_eq!(
        execution_success,
        ToolboxEndpointExecution {
            processed_time: Some(SystemTime::UNIX_EPOCH + Duration::from_secs(1728376873)),
            slot: 331437116,
            payer: Pubkey::from_str("Eyh77zP5b7arPtPgpnCT8vsGmq9p5Z9HHnBSeQLnAFQi")
            .unwrap(),
            instructions: vec![Instruction {
                program_id: Pubkey::from_str(
                    "CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu"
                )
                .unwrap(),
                accounts: vec![
                    AccountMeta::new_readonly(
                        Pubkey::from_str(
                            "aca3VWxwBeu8FTZowJ9hfSKGzntjX68EXh1N9xpE1PC"
                        )
                        .unwrap(),
                        true
                    ),
                    AccountMeta::new(
                        Pubkey::from_str(
                            "UbgH7eSCxgbr7EWk3LYSA1tVCpX617oefgcgzZu5uvV"
                        )
                        .unwrap(),
                        false
                    ),
                    AccountMeta::new(
                        Pubkey::from_str(
                            "GbT1xUWY1ABi71UjjcUKbHrupYjf8nrwrijt3TjGaK2K"
                        )
                        .unwrap(),
                        false,
                    ),
                ],
                data: vec![
                    103, 14, 206, 193, 142, 223, 227, 9, 1, 0, 128, 198, 164, 126,
                    141, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 100, 1, 100, 1, 0, 1,
                    232, 17, 195, 241, 186, 207, 248, 102, 125, 229, 75, 121, 185,
                    35, 151, 130, 31, 176, 170, 150, 67, 130, 247, 239, 215, 150,
                    138, 197, 129, 249, 3, 133
                ],
            }],
            steps: Some(vec![
                ToolboxEndpointExecutionStep::Call(
                    ToolboxEndpointExecutionStepCall {
                        program_id: Pubkey::from_str(
                            "CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu"
                        )
                        .unwrap(),
                        steps: vec![
                            ToolboxEndpointExecutionStep::Log("Instruction: EditCredixLpDepository".to_string()),
                            ToolboxEndpointExecutionStep::Log("[edit_credix_lp_depository] redeemable_amount_under_management_cap 1000000000000000".to_string()),
                            ToolboxEndpointExecutionStep::Data(vec![84, 185, 158, 28, 250, 70, 21, 176, 1, 7, 17, 228, 56, 87, 90, 205, 119, 2, 84, 227, 189, 114, 85, 232, 151, 90, 250, 137, 132, 84, 150, 112, 21, 198, 93, 108, 121, 225, 245, 210, 14, 231, 178, 119, 229, 92, 74, 242, 108, 108, 129, 244, 96, 25, 107, 193, 245, 13, 207, 211, 43, 139, 165, 34, 239, 88, 7, 115, 181, 29, 5, 22, 172, 0, 128, 198, 164, 126, 141, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                            ToolboxEndpointExecutionStep::Log("[edit_credix_lp_depository] minting_fee_in_bps 100".to_string()),
                            ToolboxEndpointExecutionStep::Data(vec![28, 205, 211, 29, 30, 211, 114, 87, 1, 7, 17, 228, 56, 87, 90, 205, 119, 2, 84, 227, 189, 114, 85, 232, 151, 90, 250, 137, 132, 84, 150, 112, 21, 198, 93, 108, 121, 225, 245, 210, 14, 231, 178, 119, 229, 92, 74, 242, 108, 108, 129, 244, 96, 25, 107, 193, 245, 13, 207, 211, 43, 139, 165, 34, 239, 88, 7, 115, 181, 29, 5, 22, 172, 100]),
                            ToolboxEndpointExecutionStep::Log("[edit_credix_lp_depository] redeeming_fee_in_bps 100".to_string()),
                            ToolboxEndpointExecutionStep::Data(vec![140, 197, 83, 213, 166, 149, 209, 62, 1, 7, 17, 228, 56, 87, 90, 205, 119, 2, 84, 227, 189, 114, 85, 232, 151, 90, 250, 137, 132, 84, 150, 112, 21, 198, 93, 108, 121, 225, 245, 210, 14, 231, 178, 119, 229, 92, 74, 242, 108, 108, 129, 244, 96, 25, 107, 193, 245, 13, 207, 211, 43, 139, 165, 34, 239, 88, 7, 115, 181, 29, 5, 22, 172, 100]),
                            ToolboxEndpointExecutionStep::Log("[edit_credix_lp_depository] minting_disabled false".to_string()),
                            ToolboxEndpointExecutionStep::Data(vec![18, 2, 200, 217, 32, 255, 140, 111, 1, 7, 17, 228, 56, 87, 90, 205, 119, 2, 84, 227, 189, 114, 85, 232, 151, 90, 250, 137, 132, 84, 150, 112, 21, 198, 93, 108, 121, 225, 245, 210, 14, 231, 178, 119, 229, 92, 74, 242, 108, 108, 129, 244, 96, 25, 107, 193, 245, 13, 207, 211, 43, 139, 165, 34, 239, 88, 7, 115, 181, 29, 5, 22, 172, 0]),
                            ToolboxEndpointExecutionStep::Log("[edit_credix_lp_depository] profits_beneficiary_collateral GcuJGTE9EPaVfGQRGiMg1jXKmzWdB5vLw8XxJ9tbFM16".to_string()),
                            ToolboxEndpointExecutionStep::Data(vec![41, 136, 149, 205, 59, 53, 165, 111, 1, 7, 17, 228, 56, 87, 90, 205, 119, 2, 84, 227, 189, 114, 85, 232, 151, 90, 250, 137, 132, 84, 150, 112, 21, 198, 93, 108, 121, 225, 245, 210, 14, 231, 178, 119, 229, 92, 74, 242, 108, 108, 129, 244, 96, 25, 107, 193, 245, 13, 207, 211, 43, 139, 165, 34, 239, 88, 7, 115, 181, 29, 5, 22, 172, 232, 17, 195, 241, 186, 207, 248, 102, 125, 229, 75, 121, 185, 35, 151, 130, 31, 176, 170, 150, 67, 130, 247, 239, 215, 150, 138, 197, 129, 249, 3, 133]),
                        ],
                        consumed: Some((23988, 200000)),
                        returns: None,
                        failure: None,
                    }
                )
            ]),
            logs: Some(vec![
                "Program CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu invoke [1]".to_string(),
                "Program log: Instruction: EditCredixLpDepository".to_string(),
                "Program log: [edit_credix_lp_depository] redeemable_amount_under_management_cap 1000000000000000".to_string(),
                "Program data: VLmeHPpGFbABBxHkOFdazXcCVOO9clXol1r6iYRUlnAVxl1seeH10g7nsnflXErybGyB9GAZa8H1Dc/TK4ulIu9YB3O1HQUWrACAxqR+jQMAAAAAAAAAAAA=".to_string(),
                "Program log: [edit_credix_lp_depository] minting_fee_in_bps 100".to_string(),
                "Program data: HM3THR7TclcBBxHkOFdazXcCVOO9clXol1r6iYRUlnAVxl1seeH10g7nsnflXErybGyB9GAZa8H1Dc/TK4ulIu9YB3O1HQUWrGQ=".to_string(),
                "Program log: [edit_credix_lp_depository] redeeming_fee_in_bps 100".to_string(),
                "Program data: jMVT1aaV0T4BBxHkOFdazXcCVOO9clXol1r6iYRUlnAVxl1seeH10g7nsnflXErybGyB9GAZa8H1Dc/TK4ulIu9YB3O1HQUWrGQ=".to_string(),
                "Program log: [edit_credix_lp_depository] minting_disabled false".to_string(),
                "Program data: EgLI2SD/jG8BBxHkOFdazXcCVOO9clXol1r6iYRUlnAVxl1seeH10g7nsnflXErybGyB9GAZa8H1Dc/TK4ulIu9YB3O1HQUWrAA=".to_string(),
                "Program log: [edit_credix_lp_depository] profits_beneficiary_collateral GcuJGTE9EPaVfGQRGiMg1jXKmzWdB5vLw8XxJ9tbFM16".to_string(),
                "Program data: KYiVzTs1pW8BBxHkOFdazXcCVOO9clXol1r6iYRUlnAVxl1seeH10g7nsnflXErybGyB9GAZa8H1Dc/TK4ulIu9YB3O1HQUWrOgRw/G6z/hmfeVLebkjl4IfsKqWQ4L379eWisWB+QOF".to_string(),
                "Program CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu consumed 23988 of 200000 compute units".to_string(),
                "Program CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu success".to_string(),
            ]),
            error: None,
            units_consumed: Some(23988),
        }
    );
    // Lookup a transaction execution that already happened and failed
    let signature_failure = Signature::from_str("3VBrBZQERLxdNjqLTzwx7TMQYbUr8ti4547CUK53WByooyJHJGmnkccw2pCQVv7D7Xi65S1E7mSFZETw6ECjxdmd").unwrap();
    let execution_failure =
        endpoint.get_execution(&signature_failure).await.unwrap();
    // Check that the execution details are correct
    assert_eq!(
        execution_failure.payer,
        Pubkey::from_str("Eyh77zP5b7arPtPgpnCT8vsGmq9p5Z9HHnBSeQLnAFQi")
            .unwrap()
    );
    assert_eq!(execution_failure.instructions.len(), 2);
    assert_eq!(execution_failure.instructions[0].accounts.len(), 6);
    assert_eq!(execution_failure.instructions[1].accounts.len(), 21);
    assert_eq!(execution_failure.slot, 356222939);
    assert_eq!(
        execution_failure.error,
        Some(TransactionError::InstructionError(
            1,
            InstructionError::Custom(3012),
        )),
    );
    assert_eq!(execution_failure.logs, Some(vec![
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [1]".to_string(),
        "Program log: Create".to_string(),
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]".to_string(),
        "Program log: Instruction: GetAccountDataSize".to_string(),
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1595 of 394635 compute units".to_string(),
        "Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA pQAAAAAAAAA=".to_string(),
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".to_string(),
        "Program 11111111111111111111111111111111 invoke [2]".to_string(),
        "Program 11111111111111111111111111111111 success".to_string(),
        "Program log: Initialize the associated token account".to_string(),
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]".to_string(),
        "Program log: Instruction: InitializeImmutableOwner".to_string(),
        "Program log: Please upgrade to SPL Token 2022 for immutable owner support".to_string(),
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1405 of 388022 compute units".to_string(),
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".to_string(),
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]".to_string(),
        "Program log: Instruction: InitializeAccount3".to_string(),
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4214 of 384141 compute units".to_string(),
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".to_string(),
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 20356 of 400000 compute units".to_string(),
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success".to_string(),
        "Program CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu invoke [1]".to_string(),
        "Program log: Instruction: MintWithCredixLpDepository".to_string(),
        "Program log: AnchorError caused by account: user_collateral. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized.".to_string(),
        "Program CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu consumed 12730 of 379644 compute units".to_string(),
        "Program CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu failed: custom program error: 0xbc4".to_string()
    ]));
    assert_eq!(execution_failure.units_consumed, Some(33086));
    // Lookup a transaction that uses lookup addresses tables
    let signature_tables= Signature::from_str("2MZyi9uezffec3YyAHpkC33r8Nmgwf3cBHKH1Y9H4EHfoKtZ8sQEKVCHF2Rwb17qQCrUDXS1u1wpNnxgz79U6yWY").unwrap();
    let execution_tables =
        endpoint.get_execution(&signature_tables).await.unwrap();
    // Check that the execution details are correct
    assert_eq!(
        execution_tables.payer,
        Pubkey::from_str("8sQEYJA7f5k3LrTDDkRDj46tWayc1fAdhurh61BtfUxF")
            .unwrap()
    );
    assert_eq!(execution_tables.instructions.len(), 50);
    for instruction_tables in execution_tables.instructions {
        assert_eq!(
            instruction_tables,
            Instruction {
                program_id: ToolboxEndpoint::SYSTEM_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new(execution_tables.payer, true),
                    AccountMeta::new(
                        instruction_tables.accounts[1].pubkey,
                        false
                    )
                ],
                data: vec![2, 0, 0, 0, 0, 152, 13, 0, 0, 0, 0, 0],
            }
        );
    }
    assert_eq!(
        execution_tables.logs,
        Some({
            let mut expected_logs = vec![];
            for _ in 0..50 {
                expected_logs.push(
                    "Program 11111111111111111111111111111111 invoke [1]"
                        .to_string(),
                );
                expected_logs.push(
                    "Program 11111111111111111111111111111111 success"
                        .to_string(),
                );
            }
            expected_logs
        })
    );
    assert_eq!(execution_tables.units_consumed, Some(7500));
}
