use serde_json::Value;

use crate::toolbox_idl_path::ToolboxIdlPath;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlInstructionBlob {
    Const {
        value: Value,
        type_flat: ToolboxIdlTypeFlat,
        type_full: ToolboxIdlTypeFull,
    },
    Arg {
        path: ToolboxIdlPath,
        type_flat: Option<ToolboxIdlTypeFlat>,
        type_full: ToolboxIdlTypeFull,
    },
    Account {
        account: Option<String>,
        path: ToolboxIdlPath,
        typing: Option<(ToolboxIdlTypeFlat, ToolboxIdlTypeFull)>,
    },
}
