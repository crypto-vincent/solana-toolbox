use std::str::FromStr;

use clap::Args;
use serde_json::json;
use solana_cli_config::Config;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdl;

use crate::toolbox_cli_error::ToolboxCliError;
use crate::toolbox_cli_utils::ToolboxCliUtils;

#[derive(Debug, Clone, Args)]
pub struct ToolboxCliCommandIdlDecompileAccountArgs {
    address: String,
    // TODO - should support loading a custom IDL ?
}

// TODO - could this be merged with execution by checking if its a valid signature or not ?
impl ToolboxCliCommandIdlDecompileAccountArgs {
    pub async fn process(
        &self,
        config: &Config,
    ) -> Result<(), ToolboxCliError> {
        let mut endpoint = ToolboxCliUtils::new_endpoint(config)?;
        let address = Pubkey::from_str(&self.address).unwrap();
        let account = endpoint.get_account(&address).await?.unwrap(); // TODO - unwrap util
        let idl = ToolboxIdl::get_for_program_id(&mut endpoint, &account.owner)
            .await?
            .unwrap(); // TODO - handle unwrap
        let decompiled = idl.decompile_account(&account.data).unwrap();
        let json = json!({
            "name": decompiled.name,
            "state": decompiled.state,
        });
        println!("{}", serde_json::to_string(&json)?);
        Ok(())
    }
}
