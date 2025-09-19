use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;
use crate::toolbox_idl_path::ToolboxIdlPath;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
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
                    ToolboxIdlInstructionAccount::try_parse_pda_blob(
                        idl_instruction_account_pda_seed,
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
                ToolboxIdlInstructionAccount::try_parse_pda_blob(
                    idl_instruction_account_pda_program,
                    typedefs,
                )
                .context("Program Id")?,
            );
        }
        Ok(Some(ToolboxIdlInstructionAccountPda { seeds, program }))
    }

    fn try_parse_pda_blob(
        idl_instruction_account_pda_blob: &Value,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_object()
        {
            if let Some(idl_instruction_account_pda_blob_value) =
                idl_instruction_account_pda_blob.get("value")
            {
                let idl_instruction_account_pda_blob_type =
                    idl_instruction_account_pda_blob
                        .get("type")
                        .cloned()
                        .unwrap_or_else(|| json!("bytes"));
                return ToolboxIdlInstructionAccount::try_parse_pda_blob_const_type_value(
                    idl_instruction_account_pda_blob_value.clone(),
                    &idl_instruction_account_pda_blob_type,
                    typedefs,
                ).context("Blob object value");
            }
            let idl_instruction_account_pda_blob_path =
                idl_object_get_key_as_str_or_else(
                    idl_instruction_account_pda_blob,
                    "path",
                )?;
            let idl_instruction_account_pda_blob_type =
                idl_instruction_account_pda_blob.get("type");
            if idl_object_get_key_as_str(
                idl_instruction_account_pda_blob,
                "kind",
            ) == Some("arg")
            {
                return ToolboxIdlInstructionAccount::try_parse_pda_blob_arg_path(
                    idl_instruction_account_pda_blob_path,
                    idl_instruction_account_pda_blob_type,
                    typedefs,
                ).context("Blob arg");
            }
            let idl_instruction_account_pda_blob_account =
                idl_object_get_key_as_str(
                    idl_instruction_account_pda_blob,
                    "account",
                );
            return ToolboxIdlInstructionAccount::try_parse_pda_blob_account_path(
                &idl_instruction_account_pda_blob_account,
                idl_instruction_account_pda_blob_path,
                idl_instruction_account_pda_blob_type,
                typedefs,
            ).context("Blob account");
        }
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_array()
        {
            return ToolboxIdlInstructionAccount::try_parse_pda_blob_const_type_value(
                json!(idl_instruction_account_pda_blob),
                &json!("bytes"),
                typedefs,
            ).context("Blob array");
        }
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_str()
        {
            return ToolboxIdlInstructionAccount::try_parse_pda_blob_const_type_value(
                json!(idl_instruction_account_pda_blob),
                &json!("string"),
                typedefs,
            ).context("Blob string");
        }
        Err(anyhow!(
            "Could not parse blob bytes (expected an object/array/string)"
        ))
    }

    fn try_parse_pda_blob_const_type_value(
        idl_instruction_account_pda_blob_value: Value,
        idl_instruction_account_pda_blob_type: &Value,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        let type_flat = ToolboxIdlTypeFlat::try_parse(
            idl_instruction_account_pda_blob_type,
        )
        .context("Parse const value type")?;
        let type_full = type_flat
            .try_hydrate(&HashMap::new(), typedefs)
            .context("Hydrate Const value type")?;
        Ok(ToolboxIdlInstructionAccountPdaBlob::Const {
            type_flat,
            type_full,
            value: idl_instruction_account_pda_blob_value,
        })
    }

    fn try_parse_pda_blob_arg_path(
        idl_instruction_account_pda_blob_path: &str,
        idl_instruction_account_pda_blob_type: Option<&Value>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        let path =
            ToolboxIdlPath::try_parse(idl_instruction_account_pda_blob_path)?;
        let typing = idl_instruction_account_pda_blob_type
            .map(|type_value| -> Result<_> {
                let type_flat = ToolboxIdlTypeFlat::try_parse(type_value)
                    .context("Parse arg value type")?;
                let type_full = type_flat
                    .try_hydrate(&HashMap::new(), typedefs)
                    .context("Hydrate arg value type")?;
                Ok((type_flat, type_full))
            })
            .transpose()?;
        Ok(ToolboxIdlInstructionAccountPdaBlob::Arg { path, typing })
    }

    fn try_parse_pda_blob_account_path(
        idl_instruction_account_pda_blob_account: &Option<&str>,
        idl_instruction_account_pda_blob_path: &str,
        idl_instruction_account_pda_blob_type: Option<&Value>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        let account = idl_instruction_account_pda_blob_account
            .map(|account| account.to_string());
        let path =
            ToolboxIdlPath::try_parse(idl_instruction_account_pda_blob_path)
                .context("Parse path")?;
        let typing = idl_instruction_account_pda_blob_type
            .map(|type_value| -> Result<_> {
                let type_flat = ToolboxIdlTypeFlat::try_parse(type_value)
                    .context("Parse account value type")?;
                let type_full = type_flat
                    .try_hydrate(&HashMap::new(), typedefs)
                    .context("Hydrate account value type")?;
                Ok((type_flat, type_full))
            })
            .transpose()?;
        Ok(ToolboxIdlInstructionAccountPdaBlob::Account {
            account,
            path,
            typing,
        })
    }
}
