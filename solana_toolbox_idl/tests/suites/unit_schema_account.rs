use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [],
                "fields": [
                    { "name": "my_struct", "type": "MyStruct" },
                    { "name": "my_enum", "type": "MyEnum" },
                ],
            },
        },
        "types": {
            "MyStruct": {
                "fields": [
                    { "name": "u8", "type": "u8" },
                    { "name": "u64", "type": "u64" },
                    { "name": "string", "type": "string" },
                    { "name": "vec_u8", "type": ["u8"] },
                    { "name": "vec_vec_u8", "type": [["u8"]] },
                    { "name": "array_u32_4", "type": ["u32", 4] },
                    { "name": "struct", "type" : {"fields": []} },
                    { "name": "enum", "type" : {"variants": []} },
                    { "name": "option_f32", "option": "f32" },
                ]
            },
            "MyEnum": {
                "variants": [
                    "Variant0",
                    {"name": "Variant1", "fields": ["u32", "u16"]},
                    {"name": "Variant2"},
                ],
            }
        },
    }))
    .unwrap();
    // Check the schema for the account
    assert_eq!(
        idl_program
            .accounts
            .get("MyAccount")
            .unwrap()
            .content_type_full
            .schema(Some("Root".to_string())),
        json!({
            "additionalProperties": false,
            "description": "Root",
            "properties": {
                "my_enum": {
                    "anyOf": [
                        { "enum": ["Variant0"] },
                        {
                            "additionalProperties": false,
                            "properties": {
                                "Variant1": {
                                    "items": [
                                        { "type": "integer" },
                                        { "type": "integer" },
                                    ],
                                    "type": "array"
                                }
                            },
                            "required": ["Variant1"],
                            "type": "object"
                        },
                        { "enum": ["Variant2"] },
                    ],
                    "description": "MyEnum"
                },
                "my_struct": {
                    "additionalProperties": false,
                    "description": "MyStruct",
                    "properties": {
                        "array_u32_4": {
                            "items": { "type": "integer" },
                            "type": "array"
                        },
                        "enum": { "type": "null" },
                        "option_f32": {
                            "anyOf": [
                                { "type": "number" },
                                { "type": "null" }
                            ]
                        },
                        "string": { "type": "string" },
                        "struct": { "type": "null" },
                        "u64": { "description": "u64", "type": "string" },
                        "u8": { "type": "integer" },
                        "vec_u8": {
                            "items": { "type": "integer" },
                            "type": "array"
                        },
                        "vec_vec_u8": {
                            "items": {
                                "items": { "type": "integer" },
                                "type": "array"
                            },
                            "type": "array"
                        }
                    },
                    "required": [
                        "array_u32_4",
                        "enum",
                        "option_f32",
                        "string",
                        "struct",
                        "u64",
                        "u8",
                        "vec_u8",
                        "vec_vec_u8"
                    ],
                    "type": "object"
                }
            },
            "required": ["my_enum", "my_struct"],
            "type": "object"
        })
    );
}
