use anyhow::Context;
use anyhow::Result;
use serde_json::Value;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_value_as_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

impl ToolboxIdlTypedef {
    pub fn try_parse(
        idl_typedef_name: &str,
        idl_typedef: &Value,
    ) -> Result<ToolboxIdlTypedef> {
        let docs = idl_typedef.get("docs").cloned();
        let mut generics = vec![];
        if let Some(idl_typedef_generics) =
            idl_value_as_object_get_key_as_array(idl_typedef, "generics")
        {
            for (_, idl_typedef_generic, context) in
                idl_iter_get_scoped_values(idl_typedef_generics)
            {
                let idl_typedef_generic_name =
                    idl_value_as_str_or_object_with_name_as_str_or_else(
                        idl_typedef_generic,
                    )
                    .context(context)?;
                generics.push(idl_typedef_generic_name.to_string());
            }
        }
        Ok(ToolboxIdlTypedef {
            name: idl_typedef_name.to_string(),
            docs,
            generics,
            type_flat: ToolboxIdlTypeFlat::try_parse_value(idl_typedef)?,
        })
    }
}
