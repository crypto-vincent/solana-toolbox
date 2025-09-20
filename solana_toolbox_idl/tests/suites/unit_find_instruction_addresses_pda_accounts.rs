use std::collections::HashMap;
use std::sync::Arc;

use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse(&json!({
        "instructions": {
            "my_ix": {
                "discriminator": [33, 34],
                "accounts": [
                    { "name": "first" },
                    {
                        "name": "pda",
                        "pda": {
                            "seeds": [
                                { "kind": "account", "path": "first" },
                                { "kind": "account", "path": "first.u8" },
                                { "kind": "account", "path": "first.u16" },
                                { "kind": "account", "path": "first.u32" },
                                { "kind": "account", "path": "first.u64" },
                                { "account": "MyAccount", "path": "first.array_u8_2" },
                                { "account": "MyAccount", "path": "first.vec_u8_3" },
                                { "account": "MyAccount", "path": "first.string" },
                                { "account": "MyAccount", "path": "first.inner.u8" },
                                { "account": "MyAccount", "path": "first.inner.u16" },
                                { "kind": "account", "path": "first.inner.u16", "type": "u8" },
                                { "kind": "account", "path": "first.inner.u16", "type": "u32" },
                            ]
                        }
                    },
                ],
            },
        },
        "accounts": {
            "MyAccount": {
                "fields": [
                    { "name": "u8", "type": "u8" },
                    { "name": "u16", "type": "u16" },
                    { "name": "u32", "type": "u32" },
                    { "name": "u64", "type": "u64" },
                    { "name": "array_u8_2", "type": ["u8", 2] },
                    { "name": "vec_u8_3", "type": ["u8"] },
                    { "name": "string", "type": "string" },
                    {
                        "name": "inner",
                        "fields": [
                            { "name": "u8", "type": "u8" },
                            { "name": "u16", "type": "u16" },
                        ]
                    },
                ]
            }
        }
    }))
    .unwrap();
    // Keys used during the test
    let dummy_first_address = Pubkey::new_unique();
    let dummy_program_id = Pubkey::new_unique();
    let dummy_seeds: &[&[u8]] = &[
        dummy_first_address.as_ref(),
        &77u8.to_le_bytes(),
        &78u16.to_le_bytes(),
        &79u32.to_le_bytes(),
        &80u64.to_le_bytes(),
        &[11u8, 12u8],
        &[21u8, 22u8, 23u8],
        b"hello",
        &111u8.to_le_bytes(),
        &222u16.to_le_bytes(),
        &222u8.to_le_bytes(),
        &222u32.to_le_bytes(),
    ];
    let dummy_pda =
        Pubkey::find_program_address(dummy_seeds, &dummy_program_id).0;
    // Assert that the accounts can be properly resolved
    let idl_account = idl_program.accounts.get("MyAccount").unwrap();
    let idl_instruction = idl_program.instructions.get("my_ix").unwrap();
    let instruction_addresses = idl_instruction.find_addresses_with_accounts(
        &dummy_program_id,
        &json!({}),
        &HashMap::from_iter([("first".to_string(), dummy_first_address)]),
        &HashMap::from_iter([(
            "first".to_string(),
            json!({
                "u8": 77,
                "u16": 78,
                "u32": 79,
                "u64": 80,
                "array_u8_2": [11, 12],
                "vec_u8_3": [21, 22, 23],
                "string": "hello",
                "inner": {
                    "u8": 111,
                    "u16": 222,
                },
            }),
        )]),
        &HashMap::from_iter([(
            "first".to_string(),
            Arc::new(idl_account.content_type_full.clone()),
        )]),
    );
    assert_eq!(*instruction_addresses.get("pda").unwrap(), dummy_pda);
}
