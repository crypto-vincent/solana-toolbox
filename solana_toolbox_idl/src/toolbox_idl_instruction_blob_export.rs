use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_format::ToolboxIdlFormat;
use crate::toolbox_idl_instruction_blob::ToolboxIdlInstructionBlob;

impl ToolboxIdlInstructionBlob {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        match self {
            ToolboxIdlInstructionBlob::Const {
                value,
                type_flat,
                type_full,
            } => {
                if format.can_skip_instruction_account_pda_object_wrap
                    && (type_full.is_vec32_u8() || type_full.is_string32())
                {
                    return json!(value);
                }
                json!({
                    "kind": "const",
                    "type": type_flat.export(format),
                    "value": value,
                })
            },
            ToolboxIdlInstructionBlob::Arg {
                path, type_flat, ..
            } => {
                let mut json_pda_blob_arg = Map::new();
                json_pda_blob_arg.insert("kind".to_string(), json!("arg"));
                json_pda_blob_arg
                    .insert("path".to_string(), json!(path.value()));
                if let Some(type_flat) = type_flat {
                    json_pda_blob_arg
                        .insert("type".to_string(), type_flat.export(format));
                }
                json!(json_pda_blob_arg)
            },
            ToolboxIdlInstructionBlob::Account {
                account,
                path,
                typing,
            } => {
                let mut json_pda_blob_account = Map::new();
                if let Some(account) = account {
                    json_pda_blob_account
                        .insert("account".to_string(), json!(account));
                }
                if !format.can_skip_instruction_account_pda_account_kind {
                    json_pda_blob_account
                        .insert("kind".to_string(), json!("account"));
                }
                json_pda_blob_account
                    .insert("path".to_string(), json!(path.value()));
                if let Some(typing) = typing {
                    json_pda_blob_account
                        .insert("type".to_string(), typing.0.export(format));
                }
                json!(json_pda_blob_account)
            },
        }
    }
}
