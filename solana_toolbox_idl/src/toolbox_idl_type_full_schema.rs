use std::vec;

use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;

impl ToolboxIdlTypeFull {
    pub fn schema(&self, description: Option<String>) -> Value {
        match self {
            ToolboxIdlTypeFull::Typedef { content, name, .. } => {
                content.schema(Some(name.clone()))
            },
            ToolboxIdlTypeFull::Option { content, .. } => {
                json!({
                    "anyOf": [
                        content.schema(None),
                        {"type": "null"},
                    ]
                })
            },
            ToolboxIdlTypeFull::Vec { items, .. } => {
                json!({
                    "type": "array",
                    "items": items.schema(None)
                })
            },
            ToolboxIdlTypeFull::Array { items, length } => {
                json!({
                    "type": "array",
                    "items": items.schema(None),
                    "minItems": length,
                    "maxItems": length
                })
            },
            ToolboxIdlTypeFull::String { .. } => json!({ "type": "string" }),
            ToolboxIdlTypeFull::Struct { fields, .. } => {
                fields.schema(description)
            },
            ToolboxIdlTypeFull::Enum { variants, .. } => {
                let mut json_variants_strings = vec![];
                let mut json_variants_objects = vec![];
                for variant in variants {
                    if variant.fields.is_empty() {
                        json_variants_strings.push(variant.name.to_string());
                        json_variants_objects.push(json!({
                            "enum": [variant.name],
                        }));
                    } else {
                        json_variants_objects.push(json!({
                            "type": "object",
                            "properties": {
                                variant.name.to_string(): variant.fields.schema(None),
                            },
                            "required": [variant.name],
                            "additionalProperties": false
                        }));
                    }
                }
                if json_variants_strings.len() == variants.len() {
                    let mut json_object = Map::new();
                    if let Some(description) = description {
                        json_object.insert(
                            "description".to_string(),
                            json!(description),
                        );
                    }
                    json_object.insert(
                        "enum".to_string(),
                        json!(json_variants_strings),
                    );
                    json!(json_object)
                } else {
                    let mut json_object = Map::new();
                    if let Some(description) = description {
                        json_object.insert(
                            "description".to_string(),
                            json!(description),
                        );
                    }
                    json_object.insert(
                        "anyOf".to_string(),
                        json!(json_variants_objects),
                    );
                    json!(json_object)
                }
            },
            ToolboxIdlTypeFull::Padded { content, .. } => {
                content.schema(description)
            },
            ToolboxIdlTypeFull::Const { literal } => {
                json!(literal) // TODO - this makes no sense
            },
            ToolboxIdlTypeFull::Primitive { primitive } => match primitive {
                ToolboxIdlTypePrimitive::U8
                | ToolboxIdlTypePrimitive::U16
                | ToolboxIdlTypePrimitive::U32
                | ToolboxIdlTypePrimitive::U64
                | ToolboxIdlTypePrimitive::U128
                | ToolboxIdlTypePrimitive::I8
                | ToolboxIdlTypePrimitive::I16
                | ToolboxIdlTypePrimitive::I32
                | ToolboxIdlTypePrimitive::I64
                | ToolboxIdlTypePrimitive::I128
                | ToolboxIdlTypePrimitive::F32
                | ToolboxIdlTypePrimitive::F64 => {
                    json!({
                        "description": primitive.as_str(),
                        "type": "number",
                    })
                },
                ToolboxIdlTypePrimitive::Bool => {
                    json!({ "type": "boolean" })
                },
                ToolboxIdlTypePrimitive::Pubkey => {
                    json!({
                        "description": "Pubkey (base58)",
                        "type": "string",
                    })
                },
            },
        }
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn schema(&self, description: Option<String>) -> Value {
        if self.is_empty() {
            let mut json_object = Map::new();
            if let Some(description) = description {
                json_object
                    .insert("description".to_string(), json!(description));
            }
            json_object.insert("type".to_string(), json!("null"));
            return json!(json_object);
        }
        match self {
            ToolboxIdlTypeFullFields::Named(fields) => {
                let mut json_properties = Map::new();
                for field in fields {
                    json_properties.insert(
                        field.name.to_string(),
                        field.content.schema(None),
                    );
                }
                let mut json_object = Map::new();
                if let Some(description) = description {
                    json_object
                        .insert("description".to_string(), json!(description));
                }
                json_object.insert("type".to_string(), json!("object"));
                json_object
                    .insert("properties".to_string(), json!(json_properties));
                json_object.insert(
                    "required".to_string(),
                    json!(json_properties.keys().collect::<Vec<_>>()),
                );
                json_object
                    .insert("additionalProperties".to_string(), json!(false));
                json!(json_object)
            },
            ToolboxIdlTypeFullFields::Unnamed(fields) => {
                let mut json_fields = vec![];
                for field in fields {
                    json_fields.push(field.content.schema(None));
                }
                let mut json_object = Map::new();
                if let Some(description) = description {
                    json_object
                        .insert("description".to_string(), json!(description));
                }
                json_object.insert("type".to_string(), json!("array"));
                json_object.insert("items".to_string(), json!(json_fields));
                json_object.insert("minItems".to_string(), json!(fields.len()));
                json_object.insert("maxItems".to_string(), json!(fields.len()));
                json!(json_object)
            },
        }
    }
}
