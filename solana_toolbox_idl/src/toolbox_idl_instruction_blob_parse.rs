use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::json;
use serde_json::Value;

use crate::toolbox_idl_instruction_blob::ToolboxIdlInstructionBlob;
use crate::toolbox_idl_path::ToolboxIdlPath;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;

impl ToolboxIdlInstructionBlob {
    pub fn try_parse(
        idl_instruction_blob: &Value,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstructionBlob> {
        if let Some(idl_instruction_blob) = idl_instruction_blob.as_object() {
            if let Some(idl_instruction_blob_value) =
                idl_instruction_blob.get("value")
            {
                let idl_instruction_blob_type = idl_instruction_blob
                    .get("type")
                    .cloned()
                    .unwrap_or_else(|| json!("bytes"));
                return ToolboxIdlInstructionBlob::try_parse_const(
                    idl_instruction_blob_value.clone(),
                    &idl_instruction_blob_type,
                    typedefs,
                )
                .context("Blob object value");
            }
            let idl_instruction_blob_path = idl_object_get_key_as_str_or_else(
                idl_instruction_blob,
                "path",
            )?;
            let idl_instruction_blob_type = idl_instruction_blob.get("type");
            if idl_object_get_key_as_str(idl_instruction_blob, "kind")
                == Some("arg")
            {
                return ToolboxIdlInstructionBlob::try_parse_arg(
                    idl_instruction_blob_path,
                    idl_instruction_blob_type,
                    typedefs,
                )
                .context("Blob arg");
            }
            let idl_instruction_blob_account =
                idl_object_get_key_as_str(idl_instruction_blob, "account");
            return ToolboxIdlInstructionBlob::try_parse_account(
                &idl_instruction_blob_account,
                idl_instruction_blob_path,
                idl_instruction_blob_type,
                typedefs,
            )
            .context("Blob account");
        }
        if let Some(idl_instruction_blob) = idl_instruction_blob.as_array() {
            return ToolboxIdlInstructionBlob::try_parse_const(
                json!(idl_instruction_blob),
                &json!("bytes"),
                typedefs,
            )
            .context("Blob array");
        }
        if let Some(idl_instruction_blob) = idl_instruction_blob.as_str() {
            return ToolboxIdlInstructionBlob::try_parse_const(
                json!(idl_instruction_blob),
                &json!("string"),
                typedefs,
            )
            .context("Blob string");
        }
        Err(anyhow!(
            "Could not parse blob bytes (expected an object/array/string)"
        ))
    }

    fn try_parse_const(
        idl_instruction_blob_value: Value,
        idl_instruction_blob_type: &Value,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstructionBlob> {
        let type_flat =
            ToolboxIdlTypeFlat::try_parse(idl_instruction_blob_type)
                .context("Parse const value type")?;
        let type_full = type_flat
            .try_hydrate(&HashMap::new(), typedefs)
            .context("Hydrate Const value type")?;
        Ok(ToolboxIdlInstructionBlob::Const {
            type_flat,
            type_full,
            value: idl_instruction_blob_value,
        })
    }

    fn try_parse_arg(
        idl_instruction_blob_path: &str,
        idl_instruction_blob_type: Option<&Value>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstructionBlob> {
        let path = ToolboxIdlPath::try_parse(idl_instruction_blob_path)?;
        let typing = idl_instruction_blob_type
            .map(|type_value| -> Result<_> {
                let type_flat = ToolboxIdlTypeFlat::try_parse(type_value)
                    .context("Parse arg value type")?;
                let type_full = type_flat
                    .try_hydrate(&HashMap::new(), typedefs)
                    .context("Hydrate arg value type")?;
                Ok((type_flat, type_full))
            })
            .transpose()?;
        Ok(ToolboxIdlInstructionBlob::Arg { path, typing })
    }

    fn try_parse_account(
        idl_instruction_blob_account: &Option<&str>,
        idl_instruction_blob_path: &str,
        idl_instruction_blob_type: Option<&Value>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstructionBlob> {
        let account =
            idl_instruction_blob_account.map(|account| account.to_string());
        let path = ToolboxIdlPath::try_parse(idl_instruction_blob_path)
            .context("Parse path")?;
        let typing = idl_instruction_blob_type
            .map(|type_value| -> Result<_> {
                let type_flat = ToolboxIdlTypeFlat::try_parse(type_value)
                    .context("Parse account value type")?;
                let type_full = type_flat
                    .try_hydrate(&HashMap::new(), typedefs)
                    .context("Hydrate account value type")?;
                Ok((type_flat, type_full))
            })
            .transpose()?;
        Ok(ToolboxIdlInstructionBlob::Account {
            account,
            path,
            typing,
        })
    }
}
