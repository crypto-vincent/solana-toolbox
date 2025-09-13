use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatEnumVariant;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFieldNamed;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFieldUnnamed;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullEnumVariant;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFieldNamed;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFieldUnnamed;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullOrConstLiteral;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

impl ToolboxIdlTypeFlat {
    pub fn try_hydrate(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFullOrConstLiteral>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFull> {
        match self.try_hydrate_or_const_literal(generics_by_symbol, typedefs)? {
            ToolboxIdlTypeFullOrConstLiteral::ToolboxIdlTypeFull(type_full) => {
                Ok(type_full)
            },
            ToolboxIdlTypeFullOrConstLiteral::ConstLiteral(literal) => Err(
                anyhow!("Const literal is not a standalone type: {}", literal),
            ),
        }
    }

    pub fn try_hydrate_or_const_literal(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFullOrConstLiteral>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFullOrConstLiteral> {
        Ok(ToolboxIdlTypeFullOrConstLiteral::ToolboxIdlTypeFull(
            match self {
                ToolboxIdlTypeFlat::Defined { name, generics } => {
                    let typedef = idl_map_get_key_or_else(typedefs, name)
                        .context("Defined: Lookup")?;
                    if generics.len() < typedef.generics.len() {
                        return Err(anyhow!(
                        "Defined: Insufficient number of generic parameter for {}: expected: {}, found: {}",
                        typedef.name,
                        typedef.generics.len(),
                        generics.len()
                    ));
                    }
                    let generics_by_symbol = typedef
                        .generics
                        .iter()
                        .enumerate()
                        .map(|(index, generic_name)| {
                            generics
                                .get(index)
                                .unwrap()
                                .try_hydrate_or_const_literal(
                                    generics_by_symbol,
                                    typedefs,
                                )
                                .with_context(|| {
                                    format!(
                                        "Defined: {}, Generic[{}]",
                                        name, generic_name
                                    )
                                })
                                .map(|generic_type| {
                                    (generic_name.to_string(), generic_type)
                                })
                        })
                        .collect::<Result<
                            HashMap<String, ToolboxIdlTypeFullOrConstLiteral>,
                        >>()?;
                    let type_full = typedef
                        .type_flat
                        .try_hydrate(&generics_by_symbol, typedefs)
                        .with_context(|| {
                            format!("Defined: {}, Content", name)
                        })?;
                    match &typedef.serialization {
                        Some(serialization) if serialization == "bytemuck" => {
                            type_full
                                .bytemuck(&typedef.name, &typedef.repr)
                                .with_context(|| {
                                    format!("Defined: {}, Bytemuck", name)
                                })?
                                .2
                        },
                        _ => ToolboxIdlTypeFull::Typedef {
                            name: typedef.name.clone(),
                            repr: typedef.repr.clone(),
                            content: Box::new(type_full),
                        },
                    }
                },
                ToolboxIdlTypeFlat::Generic { symbol } => {
                    return Ok(idl_map_get_key_or_else(
                        generics_by_symbol,
                        symbol,
                    )
                    .with_context(|| format!("Generic Lookup: {}", symbol))?
                    .clone())
                },
                ToolboxIdlTypeFlat::Option { prefix, content } => {
                    ToolboxIdlTypeFull::Option {
                        prefix: prefix.clone(),
                        content: Box::new(
                            content
                                .try_hydrate(generics_by_symbol, typedefs)?,
                        ),
                    }
                },
                ToolboxIdlTypeFlat::Vec { prefix, items } => {
                    ToolboxIdlTypeFull::Vec {
                        prefix: prefix.clone(),
                        items: Box::new(
                            items.try_hydrate(generics_by_symbol, typedefs)?,
                        ),
                    }
                },
                ToolboxIdlTypeFlat::Array { items, length } => {
                    ToolboxIdlTypeFull::Array {
                        items: Box::new(
                            items.try_hydrate(generics_by_symbol, typedefs)?,
                        ),
                        length: match length.try_hydrate_or_const_literal(
                            generics_by_symbol,
                            typedefs,
                        )? {
                            ToolboxIdlTypeFullOrConstLiteral::ConstLiteral(
                                literal,
                            ) => {
                                usize::try_from(literal).with_context(|| {
                                    format!(
                                        "Array: length: invalid: {}",
                                        literal
                                    )
                                })?
                            },
                            _ => {
                                return Err(anyhow!(
                                    "Array: length must be a const literal"
                                ))
                            },
                        },
                    }
                },
                ToolboxIdlTypeFlat::String { prefix } => {
                    ToolboxIdlTypeFull::String {
                        prefix: prefix.clone(),
                    }
                },
                ToolboxIdlTypeFlat::Struct { fields } => {
                    ToolboxIdlTypeFull::Struct {
                        fields: fields
                            .try_hydrate(generics_by_symbol, typedefs)?,
                    }
                },
                ToolboxIdlTypeFlat::Enum { prefix, variants } => {
                    ToolboxIdlTypeFull::Enum {
                        prefix: prefix.clone(),
                        variants: variants
                            .iter()
                            .map(|variant| {
                                variant
                                    .try_hydrate(generics_by_symbol, typedefs)
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                    }
                },
                ToolboxIdlTypeFlat::Padded {
                    before,
                    min_size,
                    after,
                    content,
                } => ToolboxIdlTypeFull::Padded {
                    before: *before,
                    min_size: *min_size,
                    after: *after,
                    content: Box::new(
                        content.try_hydrate(generics_by_symbol, typedefs)?,
                    ),
                },
                ToolboxIdlTypeFlat::Const { literal } => {
                    return Ok(ToolboxIdlTypeFullOrConstLiteral::ConstLiteral(
                        *literal,
                    ))
                },
                ToolboxIdlTypeFlat::Primitive { primitive } => {
                    ToolboxIdlTypeFull::Primitive {
                        primitive: primitive.clone(),
                    }
                },
            },
        ))
    }
}

impl ToolboxIdlTypeFlatEnumVariant {
    pub fn try_hydrate(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFullOrConstLiteral>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFullEnumVariant> {
        let variant_full_fields = self
            .fields
            .try_hydrate(generics_by_symbol, typedefs)
            .with_context(|| {
                format!("Variant: {}({})", self.name, self.code)
            })?;
        Ok(ToolboxIdlTypeFullEnumVariant {
            name: self.name.to_string(),
            code: self.code,
            fields: variant_full_fields,
        })
    }
}

impl ToolboxIdlTypeFlatFields {
    pub fn try_hydrate(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFullOrConstLiteral>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFullFields> {
        Ok(match self {
            ToolboxIdlTypeFlatFields::Nothing => {
                ToolboxIdlTypeFullFields::Nothing
            },
            ToolboxIdlTypeFlatFields::Named(fields) => {
                let mut fields_full = vec![];
                for field in fields {
                    fields_full
                        .push(field.try_hydrate(generics_by_symbol, typedefs)?);
                }
                ToolboxIdlTypeFullFields::Named(fields_full)
            },
            ToolboxIdlTypeFlatFields::Unnamed(fields) => {
                let mut fields_type_full = vec![];
                for (index, field) in fields.iter().enumerate() {
                    fields_type_full.push(field.try_hydrate(
                        index,
                        generics_by_symbol,
                        typedefs,
                    )?);
                }
                ToolboxIdlTypeFullFields::Unnamed(fields_type_full)
            },
        })
    }
}

impl ToolboxIdlTypeFlatFieldNamed {
    pub fn try_hydrate(
        &self,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFullOrConstLiteral>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFullFieldNamed> {
        Ok(ToolboxIdlTypeFullFieldNamed {
            name: self.name.to_string(),
            content: self
                .content
                .try_hydrate(generics_by_symbol, typedefs)
                .with_context(|| format!("Named Field: {}", self.name))?,
        })
    }
}

impl ToolboxIdlTypeFlatFieldUnnamed {
    pub fn try_hydrate(
        &self,
        index: usize,
        generics_by_symbol: &HashMap<String, ToolboxIdlTypeFullOrConstLiteral>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFullFieldUnnamed> {
        Ok(ToolboxIdlTypeFullFieldUnnamed {
            position: index,
            content: self
                .content
                .try_hydrate(generics_by_symbol, typedefs)
                .with_context(|| format!("Unnamed Field: {}", index))?,
        })
    }
}
