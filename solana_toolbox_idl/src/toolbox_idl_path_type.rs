use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use crate::toolbox_idl_path::ToolboxIdlPath;
use crate::toolbox_idl_path::ToolboxIdlPathPart;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;

impl ToolboxIdlPath {
    pub fn try_get_type_full<'a>(
        &self,
        type_full: &'a ToolboxIdlTypeFull,
    ) -> Result<&'a ToolboxIdlTypeFull> {
        let Some((current, next)) = self.split_first() else {
            return Ok(type_full);
        };
        match type_full {
            ToolboxIdlTypeFull::Typedef { content, .. } => {
                self.try_get_type_full(content)
            },
            ToolboxIdlTypeFull::Option { content, .. } => {
                self.try_get_type_full(content)
            },
            ToolboxIdlTypeFull::Vec { items, .. } => {
                if let ToolboxIdlPathPart::Key(key) = current {
                    return Err(anyhow!(
                        "Vec cannot be accessed by key: {}",
                        key
                    ));
                }
                next.try_get_type_full(items)
            },
            ToolboxIdlTypeFull::Array { items, .. } => {
                if let ToolboxIdlPathPart::Key(key) = current {
                    return Err(anyhow!(
                        "Array cannot be accessed by key: {}",
                        key
                    ));
                }
                next.try_get_type_full(items)
            },
            ToolboxIdlTypeFull::String { .. } => Err(anyhow!(
                "Type string does not contain path: {}",
                self.value()
            )),
            ToolboxIdlTypeFull::Struct { fields } => {
                self.try_get_type_full_fields(fields)
            },
            ToolboxIdlTypeFull::Enum { variants, .. } => match current {
                ToolboxIdlPathPart::Empty => {
                    Err(anyhow!("Invalid Enum Variant: Empty String"))
                },
                ToolboxIdlPathPart::Index(index) => {
                    for variant in variants {
                        if variant.code == u128::try_from(index)? {
                            return next
                                .try_get_type_full_fields(&variant.fields);
                        }
                    }
                    Err(anyhow!("Could not find enum variant: {}", index))
                },
                ToolboxIdlPathPart::Key(key) => {
                    for variant in variants {
                        if variant.name == key {
                            return next
                                .try_get_type_full_fields(&variant.fields);
                        }
                    }
                    Err(anyhow!("Could not find enum variant: {}", key))
                },
            },
            ToolboxIdlTypeFull::Padded { content, .. } => {
                self.try_get_type_full(content)
            },
            ToolboxIdlTypeFull::Primitive { .. } => Err(anyhow!(
                "Type primitive does not contain path: {}",
                self.value()
            )),
        }
    }

    pub fn try_get_type_full_fields<'a>(
        &self,
        type_full_fields: &'a ToolboxIdlTypeFullFields,
    ) -> Result<&'a ToolboxIdlTypeFull> {
        let Some((current, next)) = self.split_first() else {
            return Err(anyhow!("Fields cannot be a standalone type"));
        };
        match type_full_fields {
            ToolboxIdlTypeFullFields::Nothing => {
                Err(anyhow!("Type has no fields: {}", self.value()))
            },
            ToolboxIdlTypeFullFields::Named(fields) => {
                let key = current.value();
                for field in fields {
                    if field.name == key {
                        return next.try_get_type_full(&field.content);
                    }
                }
                Err(anyhow!("Could not find named field: {}", key))
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let index = current.index().context("Field index")?;
                let length = fields.len();
                if index >= length {
                    return Err(anyhow!(
                        "Invalid field index: {} (length: {})",
                        index,
                        length
                    ));
                }
                next.try_get_type_full(&fields[index].content)
            },
        }
    }
}
