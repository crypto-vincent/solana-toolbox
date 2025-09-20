use anyhow::Result;
use clap::Args;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_cli_context::ToolboxCliContext;

#[derive(Debug, Clone, Args)]
#[command(about = "Search signatures that involve a specific account")]
pub struct ToolboxCliCommandHistoryArgs {
    #[arg(
        value_name = "PUBKEY",
        help = "The account pubkey that is involved in transactions"
    )]
    address: String,
    #[arg(
        long,
        value_name = "COUNT",
        help = "How many transactions should be read at most",
        default_value = "10"
    )]
    count: usize,
    #[arg(
        long,
        value_name = "OFFSET",
        help = "Pagination offset",
        default_value = "0"
    )]
    offset: usize,
    #[arg(
        long,
        value_name = "SIGNATURE",
        help = "Read transactions that happened earlier than this one"
    )]
    start_before: Option<String>,
    #[arg(
        long,
        value_name = "SIGNATURE",
        help = "Keep reading transactions until reaching this signature"
    )]
    rewind_until: Option<String>,
}

impl ToolboxCliCommandHistoryArgs {
    pub async fn process(&self, context: &ToolboxCliContext) -> Result<Value> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;
        let address = context.parse_key(&self.address)?.address();
        let start_before = self
            .start_before
            .as_ref()
            .map(|signature| context.parse_signature(signature))
            .transpose()?;
        let rewind_until = self
            .rewind_until
            .as_ref()
            .map(|signature| context.parse_signature(signature))
            .transpose()?;
        let signatures = endpoint
            .search_signatures(
                &address,
                self.offset + self.count,
                start_before,
                rewind_until,
            )
            .await?;
        let mut json_history = vec![];
        for signature in signatures.iter().skip(self.offset) {
            let mut json_instructions = vec![];
            let execution = endpoint.get_execution(signature).await?;
            for instruction in execution.instructions {
                match idl_service
                    .infer_and_decode_instruction(&mut endpoint, &instruction)
                    .await
                {
                    Ok(instruction_info) => {
                        let instruction_name = context
                            .compute_instruction_name(
                                &instruction_info.program,
                                &instruction_info.instruction,
                            );
                        json_instructions.push(json!({
                            "program_id": instruction.program_id.to_string(),
                            "name": instruction_name,
                            "addresses": Map::from_iter(
                                instruction_info
                                    .addresses
                                    .iter()
                                    .map(|(name, address)| (name.to_string(), json!(address.to_string())))
                            ),
                            "payload": instruction_info.payload,
                        }))
                    },
                    Err(error) => {
                        json_instructions.push(json!({
                            "program_id": instruction.program_id.to_string(),
                            "decode_error": context.compute_error_json(error),
                        }));
                    },
                };
            }
            json_history.push(json!({
                "signature": signature.to_string(),
                "slot": execution.slot,
                "success": execution.error.is_none(),
                "instructions": json_instructions,
                "explorer_url": context.compute_explorer_signature_url(signature),
            }));
        }
        Ok(json!(json_history))
    }
}
