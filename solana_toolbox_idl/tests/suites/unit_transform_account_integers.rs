use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "MyAccount": {
                "discriminator": [22],
                "fields": [
                    { "name": "u8", "type": "u8" },
                    { "name": "u16", "type": "u16" },
                    { "name": "u32", "type": "u32" },
                    { "name": "u64", "type": "u64" },
                    { "name": "u128", "type": "u128" },
                    { "name": "i8", "type": "i8" },
                    { "name": "i16", "type": "i16" },
                    { "name": "i32", "type": "i32" },
                    { "name": "i64", "type": "i64" },
                    { "name": "i128", "type": "i128" },
                ]
            }
        },
    }))
    .unwrap();
    // Choose the account
    let idl_account = idl_program.accounts.get("MyAccount").unwrap();
    // Dummy state we'll encode/decode
    let account_state_raw = json!({
        "u8": u8::MAX,
        "u16": u16::MAX,
        "u32": u32::MAX,
        "u64": u64::MAX.to_string(),
        "u128": u128::MAX.to_string(),
        "i8": i8::MIN,
        "i16": i16::MIN,
        "i32": i32::MIN,
        "i64": i64::MIN.to_string(),
        "i128": i128::MIN.to_string(),
    });
    let account_state_v1 = json!({
        "u8": 8,
        "u16": 16,
        "u32": 32,
        "u64": "64",
        "u128": "128",
        "i8": -8,
        "i16": -16,
        "i32": -32,
        "i64": "-64",
        "i128": "-128",
    });
    let account_state_v2 = json!({
        "u8": 8,
        "u16": 16,
        "u32": 32,
        "u64": 64,
        "u128": 128,
        "i8": -8,
        "i16": -16,
        "i32": -32,
        "i64": -64,
        "i128": -128,
    });
    // Check that we can properly serialize every versions
    let account_data_raw = idl_account.encode(&account_state_raw).unwrap();
    assert_eq!(
        account_data_raw,
        vec![
            vec![22],
            u8::MAX.to_le_bytes().to_vec(),
            u16::MAX.to_le_bytes().to_vec(),
            u32::MAX.to_le_bytes().to_vec(),
            u64::MAX.to_le_bytes().to_vec(),
            u128::MAX.to_le_bytes().to_vec(),
            i8::MIN.to_le_bytes().to_vec(),
            i16::MIN.to_le_bytes().to_vec(),
            i32::MIN.to_le_bytes().to_vec(),
            i64::MIN.to_le_bytes().to_vec(),
            i128::MIN.to_le_bytes().to_vec(),
        ]
        .concat(),
    );
    let account_data_v1 = idl_account.encode(&account_state_v1).unwrap();
    assert_eq!(
        account_data_v1,
        vec![
            vec![22],
            8u8.to_le_bytes().to_vec(),
            16u16.to_le_bytes().to_vec(),
            32u32.to_le_bytes().to_vec(),
            64u64.to_le_bytes().to_vec(),
            128u128.to_le_bytes().to_vec(),
            (-8i8).to_le_bytes().to_vec(),
            (-16i16).to_le_bytes().to_vec(),
            (-32i32).to_le_bytes().to_vec(),
            (-64i64).to_le_bytes().to_vec(),
            (-128i128).to_le_bytes().to_vec(),
        ]
        .concat(),
    );
    let account_data_v2 = idl_account.encode(&account_state_v2).unwrap();
    assert_eq!(account_data_v2, account_data_v1);
    // Check that we can also deserialize every versions
    assert_eq!(
        idl_account.decode(&account_data_raw).unwrap(),
        account_state_raw,
    );
    assert_eq!(
        idl_account.decode(&account_data_v1).unwrap(),
        account_state_v1,
    );
    assert_eq!(
        idl_account.decode(&account_data_v2).unwrap(),
        account_state_v1,
    );
}
