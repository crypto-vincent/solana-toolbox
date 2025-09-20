use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;

impl ToolboxIdlInstructionAccount {
    pub fn try_find(
        &self,
        instruction_program_id: &Pubkey,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_accounts_states: &HashMap<String, Value>,
        instruction_args_type_full_fields: &ToolboxIdlTypeFullFields,
        instruction_accounts_contents_type_full: &HashMap<
            String,
            Arc<ToolboxIdlTypeFull>,
        >,
    ) -> Result<Pubkey> {
        if let Some(address) = instruction_addresses.get(&self.name) {
            return Ok(*address);
        }
        if let Some(instruction_account_address) = &self.address {
            return Ok(*instruction_account_address);
        }
        if let Some(instruction_account_pda) = &self.pda {
            return instruction_account_pda
                .try_find(
                    instruction_program_id,
                    instruction_payload,
                    instruction_addresses,
                    instruction_accounts_states,
                    instruction_args_type_full_fields,
                    instruction_accounts_contents_type_full,
                )
                .context("Compute Pda");
        }
        Err(anyhow!(
            "Could not find account automatically: {} (unresolvable)",
            self.name
        ))
    }
}

impl ToolboxIdlInstructionAccountPda {
    pub fn try_find(
        &self,
        instruction_program_id: &Pubkey,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_accounts_states: &HashMap<String, Value>,
        instruction_args_type_full_fields: &ToolboxIdlTypeFullFields,
        instruction_accounts_contents_type_full: &HashMap<
            String,
            Arc<ToolboxIdlTypeFull>,
        >,
    ) -> Result<Pubkey> {
        let mut pda_seeds_bytes = vec![];
        for (index, pda_seed_blob) in self.seeds.iter().enumerate() {
            pda_seeds_bytes.push(
                pda_seed_blob
                    .try_compute(
                        instruction_payload,
                        instruction_addresses,
                        instruction_accounts_states,
                        instruction_args_type_full_fields,
                        instruction_accounts_contents_type_full,
                    )
                    .with_context(|| format!("Compute Seed Blob: {}", index))?,
            );
        }
        let pda_program_id = if let Some(pda_program_blob) = &self.program {
            let pda_program_id_bytes = pda_program_blob
                .try_compute(
                    instruction_payload,
                    instruction_addresses,
                    instruction_accounts_states,
                    instruction_args_type_full_fields,
                    instruction_accounts_contents_type_full,
                )
                .context("Compute Program Blob")?;
            Pubkey::new_from_array(
                pda_program_id_bytes
                    .try_into()
                    .map_err(|error| {
                        anyhow!("Invalid Pubkey bytes: {:?}", error)
                    })
                    .context("Compute Program")?,
            )
        } else {
            *instruction_program_id
        };
        let mut pda_seeds_slices = vec![];
        for pda_seed_bytes in pda_seeds_bytes.iter() {
            pda_seeds_slices.push(&pda_seed_bytes[..]);
        }
        Ok(Pubkey::find_program_address(&pda_seeds_slices, &pda_program_id).0)
    }
}
