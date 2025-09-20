use std::vec;

use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [77, 78],
                "fields": [
                    "MyEnum",
                    "MyEnum",
                    "MyEnum",
                    "MyEnum",
                    "MyEnum",
                ]
            },
        },
        "types": {
            "MyEnum": {
                "variants128": [
                    {
                        "name": "Named",
                        "fields": [
                            { "name": "field1", "type": "u32"},
                        ]
                    },
                    {
                        "name": "Unnamed",
                        "code": 99,
                        "fields": ["u8", "u8"],
                    },
                    {
                        "name": "Empty",
                    },
                    "Shortened",
                    {
                        "name": "BigCode",
                        "code": u128::MAX.to_string(),
                    }
                ],
            },
        },
    }))
    .unwrap();
    // MyAccount info
    let idl_account = idl_program.accounts.get("MyAccount").unwrap();
    eprintln!("{:#?}", idl_account.content_type_full);
    let account_state = json!([
        "Empty",
        {"Named": {"field1": 42}},
        {"Unnamed": [22, 23]},
        "Shortened",
        "BigCode",
    ]);
    // Check that we can use the manual IDL to encode/decode our account
    let account_data = idl_account.encode(&account_state).unwrap();
    assert_eq!(
        account_data,
        [
            vec![77, 78],
            2u128.to_le_bytes().to_vec(),
            vec![],
            0u128.to_le_bytes().to_vec(),
            vec![42, 0, 0, 0],
            99u128.to_le_bytes().to_vec(),
            vec![22, 23],
            3u128.to_le_bytes().to_vec(),
            vec![],
            u128::MAX.to_le_bytes().to_vec(),
            vec![],
        ]
        .concat()
    );
    assert_eq!(account_state, idl_account.decode(&account_data).unwrap());
}
