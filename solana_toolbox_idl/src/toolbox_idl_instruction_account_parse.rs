use std::str::FromStr;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;

impl ToolboxIdlInstructionAccount {
    pub fn try_parse(
        idl_instruction_account_index: usize,
        idl_instruction_account: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlInstructionAccount, ToolboxIdlError> {
        let idl_instruction_account =
            idl_as_object_or_else(idl_instruction_account, &breadcrumbs.idl())?;
        let idl_instruction_account_name = idl_object_get_key_as_str_or_else(
            idl_instruction_account,
            "name",
            &breadcrumbs.idl(),
        )?;
        let breadcrumbs = &breadcrumbs.with_idl(idl_instruction_account_name);
        let idl_instruction_account_is_writable =
            idl_object_get_key_as_bool(idl_instruction_account, "writable")
                .or(idl_object_get_key_as_bool(
                    idl_instruction_account,
                    "isMut",
                ))
                .unwrap_or(false);
        let idl_instruction_account_is_signer =
            idl_object_get_key_as_bool(idl_instruction_account, "signer")
                .or(idl_object_get_key_as_bool(
                    idl_instruction_account,
                    "isSigner",
                ))
                .unwrap_or(false);
        Ok(ToolboxIdlInstructionAccount {
            index: idl_instruction_account_index + 1,
            name: idl_instruction_account_name.to_string(),
            is_writable: idl_instruction_account_is_writable,
            is_signer: idl_instruction_account_is_signer,
            address: ToolboxIdlInstructionAccount::try_parse_address(
                idl_instruction_account,
                breadcrumbs,
            )?,
            pda: ToolboxIdlInstructionAccount::try_parse_pda(
                idl_instruction_account,
                breadcrumbs,
            )?,
        })
    }

    fn try_parse_address(
        idl_instruction_account: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Option<Pubkey>, ToolboxIdlError> {
        let idl_instruction_account_address =
            match idl_object_get_key_as_str(idl_instruction_account, "address")
            {
                None => return Ok(None),
                Some(val) => val,
            };
        Ok(Some(
            Pubkey::from_str(idl_instruction_account_address).map_err(
                |err| ToolboxIdlError::InvalidPubkey {
                    parsing: err,
                    context: breadcrumbs.as_idl("address"),
                },
            )?,
        ))
    }

    fn try_parse_pda(
        idl_instruction_account: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Option<ToolboxIdlInstructionAccountPda>, ToolboxIdlError> {
        let idl_instruction_account_pda = match idl_object_get_key_as_object(
            idl_instruction_account,
            "pda",
        ) {
            None => return Ok(None),
            Some(val) => val,
        };
        let mut pda_seeds = vec![];
        if let Some(idl_instruction_account_pda_seeds) =
            idl_object_get_key_as_array(idl_instruction_account_pda, "seeds")
        {
            for (_, idl_instruction_account_pda_seed, breadcrumbs) in
                idl_iter_get_scoped_values(
                    idl_instruction_account_pda_seeds,
                    breadcrumbs,
                )?
            {
                pda_seeds.push(
                    ToolboxIdlInstructionAccount::try_parse_pda_blob(
                        idl_instruction_account_pda_seed,
                        &breadcrumbs,
                    )?,
                );
            }
        }
        let mut pda_program = None;
        if let Some(idl_instruction_account_pda_program) =
            idl_instruction_account_pda.get("program")
        {
            pda_program =
                Some(ToolboxIdlInstructionAccount::try_parse_pda_blob(
                    idl_instruction_account_pda_program,
                    breadcrumbs,
                )?);
        }
        Ok(Some(ToolboxIdlInstructionAccountPda {
            seeds: pda_seeds,
            program: pda_program,
        }))
    }

    pub fn try_parse_pda_blob(
        idl_instruction_account_pda_blob: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob, ToolboxIdlError> {
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_object()
        {
            if let Some(idl_instruction_account_pda_blob_value) =
                idl_instruction_account_pda_blob.get("value")
            {
                return ToolboxIdlInstructionAccount::try_parse_pda_blob_const(
                    idl_instruction_account_pda_blob_value,
                    breadcrumbs,
                );
            }
            let idl_instruction_account_pda_blob_kind =
                idl_object_get_key_as_str_or_else(
                    idl_instruction_account_pda_blob,
                    "kind",
                    &breadcrumbs.idl(),
                )?;
            let idl_instruction_account_pda_blob_path =
                idl_object_get_key_as_str_or_else(
                    idl_instruction_account_pda_blob,
                    "path",
                    &breadcrumbs.idl(),
                )?;
            return match idl_instruction_account_pda_blob_kind {
                "account" => Ok(ToolboxIdlInstructionAccountPdaBlob::Account {
                    path: idl_instruction_account_pda_blob_path.to_string(),
                }),
                "arg" => Ok(ToolboxIdlInstructionAccountPdaBlob::Arg {
                    path: idl_instruction_account_pda_blob_path.to_string(),
                }),
                _ => idl_err("unknown blob kind", &breadcrumbs.idl()),
            };
        }
        ToolboxIdlInstructionAccount::try_parse_pda_blob_const(
            idl_instruction_account_pda_blob,
            breadcrumbs,
        )
    }

    fn try_parse_pda_blob_const(
        idl_instruction_account_pda_blob: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob, ToolboxIdlError> {
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_array()
        {
            return Ok(ToolboxIdlInstructionAccountPdaBlob::Const {
                bytes: idl_as_bytes_or_else(
                    idl_instruction_account_pda_blob,
                    &breadcrumbs.idl(),
                )?,
            });
        }
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_str()
        {
            return Ok(ToolboxIdlInstructionAccountPdaBlob::Const {
                bytes: idl_instruction_account_pda_blob.as_bytes().to_vec(),
            });
        }
        idl_err("Could not parse blob bytes", &breadcrumbs.idl())
    }
}
