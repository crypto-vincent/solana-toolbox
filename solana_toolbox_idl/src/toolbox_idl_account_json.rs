use serde_json::json;
use serde_json::Value;

use crate::toolbox_idl_account::ToolboxIdlAccount;

// TODO - this parse/json could be serde serialize/deserialize trait implementations?
impl ToolboxIdlAccount {
    pub fn as_json(&self, backward_compatibility: bool) -> Value {
        if backward_compatibility {
            json!({
                "name": self.name,
                "disriminator": self.discriminator,
                "type": self.content_type_flat.as_json(backward_compatibility),
            })
        } else {
            // TODO - what if discriminator is the default one, we can shortcut ?
            json!({
                "disriminator": self.discriminator,
                "type": self.content_type_flat.as_json(backward_compatibility),
            })
        }
    }
}
