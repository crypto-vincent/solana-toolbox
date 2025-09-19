use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_format::ToolboxIdlFormat;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;
use crate::toolbox_idl_utils::idl_convert_to_camel_case;

impl ToolboxIdlInstructionAccount {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        let mut json_object = Map::new();
        json_object.insert("name".to_string(), self.export_name(format));
        if let Some(docs) = &self.docs {
            json_object.insert("docs".to_string(), json!(docs));
        }
        if format.use_camel_case_instruction_account_flags {
            if self.signer {
                json_object.insert("isSigner".to_string(), json!(true));
            }
            if self.writable {
                json_object.insert("isMut".to_string(), json!(true));
            }
            if self.optional {
                json_object.insert("isOptional".to_string(), json!(true));
            }
        } else {
            if self.signer {
                json_object.insert("signer".to_string(), json!(true));
            }
            if self.writable {
                json_object.insert("writable".to_string(), json!(true));
            }
            if self.optional {
                json_object.insert("optional".to_string(), json!(true));
            }
        }
        if let Some(address) = &self.address {
            json_object
                .insert("address".to_string(), json!(address.to_string()));
        }
        if let Some(pda) = &self.pda {
            json_object.insert("pda".to_string(), pda.export(format));
        }
        json!(json_object)
    }

    fn export_name(&self, format: &ToolboxIdlFormat) -> Value {
        if format.use_camel_case_instruction_account_names {
            json!(idl_convert_to_camel_case(&self.name))
        } else {
            json!(self.name)
        }
    }
}

impl ToolboxIdlInstructionAccountPda {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        let mut json_object = Map::new();
        json_object.insert(
            "seeds".to_string(),
            json!(self
                .seeds
                .iter()
                .map(|blob| blob.export(format))
                .collect::<Vec<_>>()),
        );
        if let Some(program) = &self.program {
            json_object.insert("program".to_string(), program.export(format));
        }
        json!(json_object)
    }
}

impl ToolboxIdlInstructionAccountPdaBlob {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        match self {
            ToolboxIdlInstructionAccountPdaBlob::Const {
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
            ToolboxIdlInstructionAccountPdaBlob::Arg { path, typing } => {
                let mut json_pda_blob_arg = Map::new();
                json_pda_blob_arg.insert("kind".to_string(), json!("arg"));
                json_pda_blob_arg
                    .insert("path".to_string(), json!(path.value()));
                if let Some(typing) = typing {
                    json_pda_blob_arg
                        .insert("type".to_string(), typing.0.export(format));
                }
                json!(json_pda_blob_arg)
            },
            ToolboxIdlInstructionAccountPdaBlob::Account {
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
