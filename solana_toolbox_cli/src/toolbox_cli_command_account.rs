use anyhow::Result;
use clap::Args;
use serde_json::json;
use serde_json::Value;

use crate::toolbox_cli_context::ToolboxCliContext;

#[derive(Debug, Clone, Args)]
#[command(about = "Parse the content of an account using its program's IDL")]
pub struct ToolboxCliCommandAccountArgs {
    #[arg(value_name = "PUBKEY", help = "Any account's address Pubkey")]
    address: String,
}

impl ToolboxCliCommandAccountArgs {
    pub async fn process(&self, context: &ToolboxCliContext) -> Result<Value> {
        let mut endpoint = context.create_endpoint().await?;
        let mut idl_service = context.create_service().await?;
        let address = context.parse_key(&self.address)?.address();
        let account_info = idl_service
            .get_and_infer_and_decode_account(&mut endpoint, &address)
            .await?;
        Ok(json!({
            "address": address.to_string(),
            "lamports": account_info.lamports,
            "owner": account_info.owner.to_string(),
            "name": context.compute_account_name(
                &account_info.program,
                &account_info.account,
            ),
            "state": account_info.state,
            "space": account_info.space,
            "explorer_url": context.compute_explorer_address_url(&address),
        }))
    }
}
