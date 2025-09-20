use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_blob::ToolboxIdlInstructionBlob;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_convert_to_snake_case;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_value_as_object_or_else;

impl ToolboxIdlInstructionAccount {
    pub fn try_parse(
        idl_instruction_account: &Value,
        instruction_args_type_full_fields: &ToolboxIdlTypeFullFields,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstructionAccount> {
        let idl_instruction_account =
            idl_value_as_object_or_else(idl_instruction_account)?;
        let name = idl_convert_to_snake_case(
            idl_object_get_key_as_str_or_else(idl_instruction_account, "name")
                .context("Parse Name")?,
        );
        let docs = idl_instruction_account.get("docs").cloned();
        let writable =
            idl_object_get_key_as_bool(idl_instruction_account, "writable")
                .or(idl_object_get_key_as_bool(
                    idl_instruction_account,
                    "isMut",
                ))
                .unwrap_or(false);
        let signer =
            idl_object_get_key_as_bool(idl_instruction_account, "signer")
                .or(idl_object_get_key_as_bool(
                    idl_instruction_account,
                    "isSigner",
                ))
                .unwrap_or(false);
        let optional =
            idl_object_get_key_as_bool(idl_instruction_account, "optional")
                .or(idl_object_get_key_as_bool(
                    idl_instruction_account,
                    "isOptional",
                ))
                .unwrap_or(false);
        let address = ToolboxIdlInstructionAccount::try_parse_address(
            idl_instruction_account,
        )
        .with_context(|| format!("Parse {} Address", name))?;
        let pda = ToolboxIdlInstructionAccount::try_parse_pda(
            idl_instruction_account,
            instruction_args_type_full_fields,
            typedefs,
        )
        .with_context(|| format!("Parse {} Pda", name))?;
        Ok(ToolboxIdlInstructionAccount {
            name,
            docs,
            writable,
            signer,
            optional,
            address,
            pda,
        })
    }

    fn try_parse_address(
        idl_instruction_account: &Map<String, Value>,
    ) -> Result<Option<Pubkey>> {
        match idl_object_get_key_as_str(idl_instruction_account, "address") {
            None => Ok(None),
            Some(val) => Ok(Some(Pubkey::from_str(val)?)),
        }
    }

    fn try_parse_pda(
        idl_instruction_account: &Map<String, Value>,
        instruction_args_type_full_fields: &ToolboxIdlTypeFullFields,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<Option<ToolboxIdlInstructionAccountPda>> {
        let idl_instruction_account_pda = match idl_object_get_key_as_object(
            idl_instruction_account,
            "pda",
        ) {
            None => return Ok(None),
            Some(val) => val,
        };
        let mut seeds = vec![];
        if let Some(idl_instruction_account_pda_seeds) =
            idl_object_get_key_as_array(idl_instruction_account_pda, "seeds")
        {
            for (index, idl_instruction_account_pda_seed) in
                idl_instruction_account_pda_seeds.iter().enumerate()
            {
                seeds.push(
                    ToolboxIdlInstructionBlob::try_parse(
                        idl_instruction_account_pda_seed,
                        instruction_args_type_full_fields,
                        typedefs,
                    )
                    .with_context(|| format!("Seed: {}", index))?,
                );
            }
        }
        let mut program = None;
        if let Some(idl_instruction_account_pda_program) =
            idl_instruction_account_pda.get("program")
        {
            program = Some(
                ToolboxIdlInstructionBlob::try_parse(
                    idl_instruction_account_pda_program,
                    instruction_args_type_full_fields,
                    typedefs,
                )
                .context("Program Id")?,
            );
        }
        Ok(Some(ToolboxIdlInstructionAccountPda { seeds, program }))
    }
}
