use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_value_as_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramType {
    pub name: String,
    pub generics: Vec<String>,
    pub type_flat: ToolboxIdlTypeFlat,
}

impl ToolboxIdlProgramType {
    pub fn print(&self) {
        println!("----");
        if self.generics.is_empty() {
            println!("type.name: {}", self.name);
        } else {
            println!("type.name: {}<{}>", self.name, self.generics.join(","))
        }
        println!("type.type: {}", self.type_flat.describe());
    }

    pub(crate) fn try_parse(
        idl_type_name: &str,
        idl_type: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramType, ToolboxIdlError> {
        let mut program_type_generics = vec![];
        if let Some(idl_type_generics) =
            idl_value_as_object_get_key_as_array(idl_type, "generics")
        {
            for (_, idl_type_generic, breadcrumbs) in
                idl_iter_get_scoped_values(idl_type_generics, breadcrumbs)?
            {
                let idl_type_generic_name =
                    idl_value_as_str_or_object_with_name_as_str_or_else(
                        idl_type_generic,
                        &breadcrumbs.idl(),
                    )?;
                program_type_generics.push(idl_type_generic_name.to_string());
            }
        }
        Ok(ToolboxIdlProgramType {
            name: idl_type_name.to_string(),
            generics: program_type_generics,
            type_flat: ToolboxIdlTypeFlat::try_parse(idl_type, breadcrumbs)?,
        })
    }
}
