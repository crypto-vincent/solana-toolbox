use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;
use crate::toolbox_idl_utils::idl_convert_to_snake_case;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_u64_or_else;
use crate::toolbox_idl_utils::idl_value_as_object_get_key;
use crate::toolbox_idl_utils::idl_value_as_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_name_as_str_or_else;

impl ToolboxIdlTypeFlat {
    pub fn try_parse_object_is_possible(
        idl_object: &Map<String, Value>,
    ) -> bool {
        if idl_object.contains_key("type")
            || idl_object.contains_key("defined")
            || idl_object.contains_key("generic")
            || idl_object.contains_key("option")
            || idl_object.contains_key("option8")
            || idl_object.contains_key("option32")
            || idl_object.contains_key("vec")
            || idl_object.contains_key("vec8")
            || idl_object.contains_key("vec32")
            || idl_object.contains_key("array")
            || idl_object.contains_key("fields")
            || idl_object.contains_key("variants")
            || idl_object.contains_key("variants8")
            || idl_object.contains_key("variants32")
            || idl_object.contains_key("padded")
        {
            return true;
        }
        false
    }

    pub fn try_parse_value(idl_value: &Value) -> Result<ToolboxIdlTypeFlat> {
        if let Some(idl_object) = idl_value.as_object() {
            return ToolboxIdlTypeFlat::try_parse_object(idl_object);
        }
        if let Some(idl_array) = idl_value.as_array() {
            return ToolboxIdlTypeFlat::try_parse_array(idl_array);
        }
        if let Some(idl_str) = idl_value.as_str() {
            return ToolboxIdlTypeFlat::try_parse_str(idl_str);
        }
        if let Some(idl_u64) = idl_value.as_u64() {
            return ToolboxIdlTypeFlat::try_parse_u64(idl_u64);
        }
        Err(anyhow!(
            "Could not parse type value, expected: object, array, string or number",
        ))
    }

    pub fn try_parse_object(
        idl_object: &Map<String, Value>,
    ) -> Result<ToolboxIdlTypeFlat> {
        if let Some(idl_type) = idl_object.get("type") {
            return ToolboxIdlTypeFlat::try_parse_value(idl_type);
        }
        if let Some(idl_defined) = idl_object.get("defined") {
            return ToolboxIdlTypeFlat::try_parse_defined(idl_defined)
                .context("Defined");
        }
        if let Some(idl_generic_symbol) =
            idl_object_get_key_as_str(idl_object, "generic")
        {
            return ToolboxIdlTypeFlat::try_parse_generic_symbol(
                idl_generic_symbol,
            )
            .context("Generic");
        }
        if let Some(idl_option) = idl_object.get("option") {
            return ToolboxIdlTypeFlat::try_parse_option(1, idl_option)
                .context("Option");
        }
        if let Some(idl_option) = idl_object.get("option8") {
            return ToolboxIdlTypeFlat::try_parse_option(1, idl_option)
                .context("Option8");
        }
        if let Some(idl_option) = idl_object.get("option16") {
            return ToolboxIdlTypeFlat::try_parse_option(2, idl_option)
                .context("Option16");
        }
        if let Some(idl_option) = idl_object.get("option32") {
            return ToolboxIdlTypeFlat::try_parse_option(4, idl_option)
                .context("Option32");
        }
        if let Some(idl_vec) = idl_object.get("vec") {
            return ToolboxIdlTypeFlat::try_parse_vec(4, idl_vec)
                .context("Vec");
        }
        if let Some(idl_vec) = idl_object.get("vec8") {
            return ToolboxIdlTypeFlat::try_parse_vec(1, idl_vec)
                .context("Vec8");
        }
        if let Some(idl_vec) = idl_object.get("vec16") {
            return ToolboxIdlTypeFlat::try_parse_vec(2, idl_vec)
                .context("Vec16");
        }
        if let Some(idl_vec) = idl_object.get("vec32") {
            return ToolboxIdlTypeFlat::try_parse_vec(4, idl_vec)
                .context("Vec32");
        }
        if let Some(idl_array) =
            idl_object_get_key_as_array(idl_object, "array")
        {
            return ToolboxIdlTypeFlat::try_parse_array(idl_array)
                .context("Array");
        }
        if let Some(idl_struct_fields) =
            idl_object_get_key_as_array(idl_object, "fields")
        {
            return ToolboxIdlTypeFlat::try_parse_struct_fields(
                idl_struct_fields,
            )
            .context("Struct Fields");
        }
        if let Some(idl_enum_variants) =
            idl_object_get_key_as_array(idl_object, "variants")
        {
            return ToolboxIdlTypeFlat::try_parse_enum_variants(
                1,
                idl_enum_variants,
            )
            .context("Enum Variants");
        }
        if let Some(idl_enum_variants) =
            idl_object_get_key_as_array(idl_object, "variants8")
        {
            return ToolboxIdlTypeFlat::try_parse_enum_variants(
                1,
                idl_enum_variants,
            )
            .context("Enum Variants8");
        }
        if let Some(idl_enum_variants) =
            idl_object_get_key_as_array(idl_object, "variants16")
        {
            return ToolboxIdlTypeFlat::try_parse_enum_variants(
                2,
                idl_enum_variants,
            )
            .context("Enum Variants16");
        }
        if let Some(idl_enum_variants) =
            idl_object_get_key_as_array(idl_object, "variants32")
        {
            return ToolboxIdlTypeFlat::try_parse_enum_variants(
                4,
                idl_enum_variants,
            )
            .context("Enum Variants32");
        }
        if let Some(idl_padded) =
            idl_object_get_key_as_object(idl_object, "padded")
        {
            return ToolboxIdlTypeFlat::try_parse_padded(idl_padded)
                .context("Padded");
        }
        if let Some(idl_value_literal) =
            idl_object_get_key_as_str(idl_object, "value")
        {
            return ToolboxIdlTypeFlat::try_parse_const_value(
                idl_value_literal,
            )
            .context("Const Value");
        }
        Err(anyhow!(
            "Could not parse type object: Missing type object key: {:?}",
            vec![
                "defined", "generic", "option", "array", "vec", "fields",
                "variants", "padded", "value"
            ]
        ))
    }

    fn try_parse_array(idl_array: &[Value]) -> Result<ToolboxIdlTypeFlat> {
        if idl_array.len() == 1 {
            return ToolboxIdlTypeFlat::try_parse_vec(4, &idl_array[0]);
        }
        if idl_array.len() == 2 {
            return Ok(ToolboxIdlTypeFlat::Array {
                items: Box::new(
                    ToolboxIdlTypeFlat::try_parse_value(&idl_array[0])
                        .context("Items")?,
                ),
                length: Box::new(
                    ToolboxIdlTypeFlat::try_parse_value(&idl_array[1])
                        .context("Length")?,
                ),
            });
        }
        Err(anyhow!(
            "Could not parse array type, expected either [type] or [type, length] format",
        ))
    }

    fn try_parse_str(idl_str: &str) -> Result<ToolboxIdlTypeFlat> {
        if idl_str == "bytes" {
            return Ok(ToolboxIdlTypeFlat::Vec {
                prefix_bytes: 4,
                items: Box::new(ToolboxIdlTypeFlat::Primitive {
                    primitive: ToolboxIdlTypePrimitive::U8,
                }),
            });
        }
        if idl_str == "publicKey" {
            return Ok(ToolboxIdlTypeFlat::Primitive {
                primitive: ToolboxIdlTypePrimitive::PublicKey,
            });
        }
        Ok(match ToolboxIdlTypePrimitive::try_parse(idl_str) {
            Some(primitive) => ToolboxIdlTypeFlat::Primitive { primitive },
            None => ToolboxIdlTypeFlat::Defined {
                name: idl_str.to_string(),
                generics: vec![],
            },
        })
    }

    fn try_parse_u64(idl_u64: u64) -> Result<ToolboxIdlTypeFlat> {
        Ok(ToolboxIdlTypeFlat::Const { literal: idl_u64 })
    }

    fn try_parse_defined(idl_defined: &Value) -> Result<ToolboxIdlTypeFlat> {
        let defined_name =
            idl_value_as_str_or_object_with_name_as_str_or_else(idl_defined)
                .context("Parse Defined Name")?
                .to_string();
        let mut defined_generics = vec![];
        if let Some(idl_defined_generics) =
            idl_value_as_object_get_key_as_array(idl_defined, "generics")
        {
            for (index, idl_defined_generic) in
                idl_defined_generics.iter().enumerate()
            {
                defined_generics.push(
                    ToolboxIdlTypeFlat::try_parse_value(idl_defined_generic)
                        .with_context(|| {
                            format!("Parse Defined's Generic Type: {}", index)
                        })?,
                );
            }
        }
        Ok(ToolboxIdlTypeFlat::Defined {
            name: defined_name,
            generics: defined_generics,
        })
    }

    fn try_parse_generic_symbol(
        idl_generic_symbol: &str,
    ) -> Result<ToolboxIdlTypeFlat> {
        Ok(ToolboxIdlTypeFlat::Generic {
            symbol: idl_generic_symbol.to_string(),
        })
    }

    fn try_parse_const_value(
        idl_value_literal: &str,
    ) -> Result<ToolboxIdlTypeFlat> {
        Ok(ToolboxIdlTypeFlat::Const {
            literal: idl_value_literal.parse().map_err(|error| {
                anyhow!("Parse int literal error: {}", error)
            })?,
        })
    }

    fn try_parse_option(
        idl_option_prefix_bytes: u8,
        idl_option_content: &Value,
    ) -> Result<ToolboxIdlTypeFlat> {
        Ok(ToolboxIdlTypeFlat::Option {
            prefix_bytes: idl_option_prefix_bytes,
            content: Box::new(ToolboxIdlTypeFlat::try_parse_value(
                idl_option_content,
            )?),
        })
    }

    fn try_parse_vec(
        idl_vec_prefix_bytes: u8,
        idl_vec_items: &Value,
    ) -> Result<ToolboxIdlTypeFlat> {
        Ok(ToolboxIdlTypeFlat::Vec {
            prefix_bytes: idl_vec_prefix_bytes,
            items: Box::new(ToolboxIdlTypeFlat::try_parse_value(
                idl_vec_items,
            )?),
        })
    }

    fn try_parse_struct_fields(
        idl_struct_fields: &[Value],
    ) -> Result<ToolboxIdlTypeFlat> {
        Ok(ToolboxIdlTypeFlat::Struct {
            fields: ToolboxIdlTypeFlatFields::try_parse(idl_struct_fields)?,
        })
    }

    fn try_parse_enum_variants(
        idl_enum_prefix_bytes: u8,
        idl_enum_variants: &[Value],
    ) -> Result<ToolboxIdlTypeFlat> {
        let mut enum_variants = vec![];
        for (index, idl_enum_variant) in idl_enum_variants.iter().enumerate() {
            let enum_variant_name =
                idl_value_as_str_or_object_with_name_as_str_or_else(
                    idl_enum_variant,
                )
                .with_context(|| format!("Parse Enum Variant Name: {}", index))?
                .to_string();
            let enum_variant_docs =
                idl_value_as_object_get_key(idl_enum_variant, "docs").cloned();
            let enum_variant_fields = if let Some(idl_enum_variant_fields) =
                idl_value_as_object_get_key_as_array(idl_enum_variant, "fields")
            {
                ToolboxIdlTypeFlatFields::try_parse(idl_enum_variant_fields)
                    .with_context(|| {
                        format!(
                            "Parse Enum Variant Type: {}",
                            enum_variant_name
                        )
                    })?
            } else {
                ToolboxIdlTypeFlatFields::None
            };
            enum_variants.push((
                enum_variant_name,
                enum_variant_docs,
                enum_variant_fields,
            ));
        }
        Ok(ToolboxIdlTypeFlat::Enum {
            prefix_bytes: idl_enum_prefix_bytes,
            variants: enum_variants,
        })
    }

    fn try_parse_padded(
        idl_padded: &Map<String, Value>,
    ) -> Result<ToolboxIdlTypeFlat> {
        let idl_padded_size =
            idl_object_get_key_as_u64_or_else(idl_padded, "size")?;
        Ok(ToolboxIdlTypeFlat::Padded {
            size_bytes: idl_padded_size,
            content: Box::new(ToolboxIdlTypeFlat::try_parse_object(
                idl_padded,
            )?),
        })
    }
}

impl ToolboxIdlTypeFlatFields {
    pub fn try_parse(idl_fields: &[Value]) -> Result<ToolboxIdlTypeFlatFields> {
        if idl_fields.is_empty() {
            return Ok(ToolboxIdlTypeFlatFields::None);
        }
        let mut fields_named = false;
        let mut fields_info = vec![];
        for (index, idl_field) in idl_fields.iter().enumerate() {
            let field_name = idl_value_as_object_get_key(idl_field, "name")
                .and_then(|name| name.as_str())
                .map(idl_convert_to_snake_case);
            if field_name.is_some() {
                fields_named = true;
            }
            let field_name_or_index =
                field_name.unwrap_or_else(|| format!("{}", index));
            let field_docs =
                idl_value_as_object_get_key(idl_field, "docs").cloned();
            let field_type_flat =
                ToolboxIdlTypeFlat::try_parse_value(idl_field).with_context(
                    || format!("Parse Field Type: {}", field_name_or_index),
                )?;
            fields_info.push((
                field_name_or_index,
                field_docs,
                field_type_flat,
            ));
        }
        if !fields_named {
            let mut fields = vec![];
            for (_, field_docs, field_type_flat) in fields_info {
                fields.push((field_docs, field_type_flat));
            }
            Ok(ToolboxIdlTypeFlatFields::Unnamed(fields))
        } else {
            Ok(ToolboxIdlTypeFlatFields::Named(fields_info))
        }
    }
}
