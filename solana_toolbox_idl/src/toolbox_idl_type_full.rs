use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFull {
    Option {
        prefix_bytes: u8,
        content: Box<ToolboxIdlTypeFull>,
    },
    Vec {
        prefix_bytes: u8,
        items: Box<ToolboxIdlTypeFull>,
    },
    Array {
        items: Box<ToolboxIdlTypeFull>,
        length: u64,
    },
    Struct {
        fields: ToolboxIdlTypeFullFields,
    },
    Enum {
        prefix_bytes: u8,
        variants: Vec<(String, ToolboxIdlTypeFullFields)>,
    },
    Padded {
        size_bytes: u64,
        content: Box<ToolboxIdlTypeFull>,
    },
    Const {
        literal: u64,
    },
    Primitive {
        primitive: ToolboxIdlTypePrimitive,
    },
}

impl ToolboxIdlTypeFull {
    pub fn nothing() -> ToolboxIdlTypeFull {
        ToolboxIdlTypeFull::Struct {
            fields: ToolboxIdlTypeFullFields::None,
        }
    }

    pub fn is_bytes(&self) -> bool {
        match self {
            ToolboxIdlTypeFull::Vec {
                prefix_bytes,
                items,
            } => {
                *prefix_bytes == 4
                    && items.is_primitive(&ToolboxIdlTypePrimitive::U8)
            },
            _ => false,
        }
    }

    pub fn is_primitive(&self, value: &ToolboxIdlTypePrimitive) -> bool {
        match self {
            ToolboxIdlTypeFull::Primitive { primitive } => primitive == value,
            _ => false,
        }
    }
}

impl ToolboxIdlTypeFull {
    pub fn as_const_literal(&self) -> Option<&u64> {
        match self {
            ToolboxIdlTypeFull::Const { literal } => Some(literal),
            _ => None,
        }
    }

    pub fn as_struct_fields(&self) -> Option<&ToolboxIdlTypeFullFields> {
        match self {
            ToolboxIdlTypeFull::Struct { fields } => Some(fields),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlTypeFullFields {
    None,
    Named(Vec<(String, ToolboxIdlTypeFull)>),
    Unnamed(Vec<ToolboxIdlTypeFull>),
}

impl ToolboxIdlTypeFullFields {
    pub fn nothing() -> ToolboxIdlTypeFullFields {
        ToolboxIdlTypeFullFields::None
    }
}

impl ToolboxIdlTypeFullFields {
    pub fn as_named(&self) -> Option<&Vec<(String, ToolboxIdlTypeFull)>> {
        match self {
            ToolboxIdlTypeFullFields::Named(named) => Some(named),
            _ => None,
        }
    }
}
