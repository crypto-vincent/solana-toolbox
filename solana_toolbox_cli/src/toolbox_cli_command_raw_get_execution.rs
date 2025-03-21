use std::str::FromStr;

use clap::Args;
use serde_json::json;
use solana_sdk::signature::Signature;

use crate::toolbox_cli_config::ToolboxCliConfig;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Get a signature's execution outcome data")]
pub struct ToolboxCliCommandRawGetExecutionArgs {
    #[arg(help = "The transaction's execution signature")]
    signature: String,
}

impl ToolboxCliCommandRawGetExecutionArgs {
    pub async fn process(
        &self,
        config: &ToolboxCliConfig,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = config.create_endpoint()?;
        let signature = Signature::from_str(&self.signature)?;
        let execution = endpoint.get_execution(&signature).await?;
        println!(
            "{}",
            serde_json::to_string(&json!({
                "payer": execution.payer.to_string(),
                "instructions": execution.instructions.iter().map(|instruction| {
                    json!({
                        "program_id": instruction.program_id.to_string(),
                        "accounts": instruction.accounts.iter().map(|account| {
                            json!({
                                "address": account.pubkey.to_string(),
                                "is_signer": account.is_signer,
                                "is_writable": account.is_writable,
                            })
                        }).collect::<Vec<_>>(),
                        "data": instruction.data,
                    })
                }).collect::<Vec<_>>(),
                "logs": execution.logs,
                "error": execution.error,
                "return_data": execution.return_data,
                "units_consumed": execution.units_consumed,
            }))?
        );
        Ok(())
    }
}
