use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_format::ToolboxIdlFormat;

impl ToolboxIdlError {
    pub fn export(&self, format: &ToolboxIdlFormat) -> Value {
        if format.can_shortcut_error_to_number_if_no_msg && self.msg.is_empty()
        {
            return json!(self.code);
        }
        let mut json_object = Map::new();
        if !format.use_object_for_unordered_named_array {
            json_object.insert("name".to_string(), json!(self.name));
        }
        json_object.insert("code".to_string(), json!(self.code));
        json_object.insert("msg".to_string(), json!(self.msg));
        json!(json_object)
    }
}
