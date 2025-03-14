use std::collections::HashMap;
use std::str::FromStr;

use clap::Args;
use clap::ValueHint;
use serde_json::from_str;
use serde_json::json;
use serde_json::Value;
use solana_cli_config::Config;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_idl::ToolboxIdl;
use solana_toolbox_idl::ToolboxIdlTransactionInstruction;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlProcessInstructionArgs {
    program_address: String,
    name: String,
    args: String,
    #[arg(value_delimiter(','))]
    accounts: Vec<String>,
    #[arg(short, long, value_hint(ValueHint::FilePath))]
    payer: Option<String>,
}

impl ToolboxCliCommandIdlProcessInstructionArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
        let payer = ToolboxCliUtils::load_keypair(
            self.payer.as_ref().unwrap_or(&config.keypair_path),
        )?;

        let program_address = Pubkey::from_str(&self.program_address).unwrap();
        let idl =
            ToolboxIdl::get_for_program_id(&mut endpoint, &program_address)
                .await?
                .unwrap(); // TODO - handle unwrap

        let args = from_str::<Value>(&self.args)?;

        let mut accounts = HashMap::new();
        accounts.insert(
            "payer".to_string(),
            KeypairOrPubkey::Keypair(payer.insecure_clone()),
        );
        for account in &self.accounts {
            let parts = account.split(":").collect::<Vec<_>>();
            if let [key, value] = parts[..] {
                accounts.insert(key.to_string(), parse_account(value)?);
            } else {
                return Err(ToolboxCliError::Custom(
                    "Invalid account key-value".to_string(),
                ));
            }
        }

        let mut accounts_addresses = HashMap::new();
        for account in &accounts {
            accounts_addresses.insert(
                account.0.to_string(),
                match account.1 {
                    KeypairOrPubkey::Keypair(keypair) => keypair.pubkey(),
                    KeypairOrPubkey::Pubkey(pubkey) => *pubkey,
                },
            );
        }

        let instruction = idl
            .resolve_instruction(
                &mut endpoint,
                &ToolboxIdlTransactionInstruction {
                    program_id: program_address,
                    name: self.name.to_string(),
                    accounts_addresses,
                    args,
                },
            )
            .await?;

        let mut signers = vec![];
        for account in &accounts {
            if let KeypairOrPubkey::Keypair(keypair) = account.1 {
                signers.push(keypair);
            }
        }

        let (signature, execution) = endpoint
            .process_instruction_with_signers(&payer, instruction, &signers)
            .await?;

        let json_execution_instructions = execution
            .instructions
            .iter()
            .map(|instruction| {
                json!({
                    "program_id": instruction.program_id.to_string(),
                    "accounts": instruction.accounts.iter()
                        .map(|account| {
                            json!({
                                "address": account.pubkey.to_string(),
                                "is_signer": account.is_signer,
                                "is_writable": account.is_writable,
                            })
                        })
                        .collect::<Vec<_>>(),
                    "data": instruction.data,
                })
            })
            .collect::<Vec<_>>();

        let json = json!({
            "signature": signature.to_string(),
            "execution": {
                "payer": execution.payer.to_string(),
                "instructions": json_execution_instructions,
                "logs": execution.logs,
                "error": execution.error,
                "return_data": execution.return_data,
                "units_consumed": execution.units_consumed,
            }
        });
        println!("{}", serde_json::to_string(&json)?);
        Ok(())
    }
}

enum KeypairOrPubkey {
    Keypair(Keypair),
    Pubkey(Pubkey),
}

fn parse_account(value: &str) -> Result<KeypairOrPubkey, ToolboxCliError> {
    Ok(if let Ok(keypair) = read_keypair_file(value) {
        KeypairOrPubkey::Keypair(keypair)
    } else {
        KeypairOrPubkey::Pubkey(Pubkey::from_str(value)?)
    })
}
