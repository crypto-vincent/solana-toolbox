use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_format::ToolboxIdlFormat;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
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
