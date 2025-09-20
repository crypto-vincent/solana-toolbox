use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_instruction_blob::ToolboxIdlInstructionBlob;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

impl ToolboxIdlInstructionBlob {
    pub fn try_compute(
        &self,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_accounts_states: &HashMap<String, Value>,
        instruction_accounts_contents_type_full: &HashMap<
            String,
            Arc<ToolboxIdlTypeFull>,
        >,
    ) -> Result<Vec<u8>> {
        let mut data = vec![];
        let prefixed = false;
        match self {
            ToolboxIdlInstructionBlob::Const {
                value, type_full, ..
            } => {
                type_full
                    .try_serialize(value, &mut data, prefixed)
                    .context("Serialize const bytes")?;
            },
            ToolboxIdlInstructionBlob::Arg {
                path, type_full, ..
            } => {
                let value = path
                    .try_get_json_value(instruction_payload)
                    .context("Extract arg value")?;
                type_full
                    .try_serialize(value, &mut data, prefixed)
                    .context("Serialize arg bytes")?;
            },
            ToolboxIdlInstructionBlob::Account { path, typing, .. } => {
                let (instruction_account_name, account_content_path) =
                    path.split_first().ok_or_else(|| {
                        anyhow!("PDA Blob account path is empty (should have at least the account name)")
                    })?;
                let instruction_account_name =
                    instruction_account_name.key().context("Account name")?;
                if account_content_path.is_empty() {
                    return idl_map_get_key_or_else(
                        instruction_addresses,
                        instruction_account_name,
                    )
                    .context("Instruction addresses")
                    .map(|address| address.to_bytes().to_vec());
                }
                let instruction_account_state = idl_map_get_key_or_else(
                    instruction_accounts_states,
                    instruction_account_name,
                )
                .context("Instruction accounts states")?;
                let value = account_content_path
                    .try_get_json_value(instruction_account_state)
                    .context("Account extract value")?;
                if let Some(typing) = typing {
                    typing
                        .1
                        .try_serialize(value, &mut data, prefixed)
                        .context("Serialize Blob Bytes")?;
                } else {
                    let instruction_account_content_type_full =
                        instruction_accounts_contents_type_full
                            .get(instruction_account_name)
                            .context("Account content type")?;
                    account_content_path
                        .try_get_type_full(
                            instruction_account_content_type_full,
                        )
                        .context("Account content type from path")?
                        .try_serialize(value, &mut data, prefixed)
                        .context("Serialize Blob Bytes")?;
                };
            },
        };
        Ok(data)
    }
}
