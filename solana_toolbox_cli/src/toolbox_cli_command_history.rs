use clap::Args;
use serde_json::json;
use serde_json::Value;

use crate::toolbox_cli_context::ToolboxCliContext;
use crate::toolbox_cli_error::ToolboxCliError;

#[derive(Debug, Clone, Args)]
#[command(about = "Search signatures that involve a specific account")]
pub struct ToolboxCliCommandHistoryArgs {
    #[arg(
        default_value = "KEYPAIR",
        help = "The account pubkey that is involved in transactions"
    )]
    address: String,
    #[arg(help = "How much signature we'll search for before stopping")]
    limit: Option<usize>,
    #[arg()]
    start_before_signature: Option<String>,
    #[arg()]
    rewind_until_signature: Option<String>,
}

impl ToolboxCliCommandHistoryArgs {
    pub async fn process(
        &self,
        context: &ToolboxCliContext,
    ) -> Result<Value, ToolboxCliError> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;
        let address = context.parse_key(&self.address)?.address();
        let start_before = self
            .start_before_signature
            .as_ref()
            .map(|signature| context.parse_signature(signature))
            .transpose()?;
        let rewind_until = self
            .rewind_until_signature
            .as_ref()
            .map(|signature| context.parse_signature(signature))
            .transpose()?;
        let signatures = endpoint
            .search_signatures(
                &address,
                start_before,
                rewind_until,
                self.limit.unwrap_or(5),
            )
            .await?;
        let mut json_history = vec![];
        for signature in signatures {
            let mut json_instructions = vec![];
            let execution = endpoint.get_execution(&signature).await?;
            for instruction in execution.instructions {
                let instruction_decoded = idl_service
                    .decode_instruction(&mut endpoint, &instruction)
                    .await?;
                json_instructions.push(context.compute_instruction_kind(
                    &instruction.program_id,
                    &instruction_decoded.program,
                    &instruction_decoded.instruction,
                ));
            }
            json_history.push(json!({
                "signature": signature.to_string(),
                "instructions": json_instructions,
                "explorer": context.compute_explorer_signature_link(&signature),
            }));
        }
        Ok(json!(json_history))
    }
}
