use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "MyAccount": {
                "fields": [
                    { "name": "bytes", "type": "bytes" },
                    { "name": "vec_u8", "type": {"vec": "u8"} },
                    { "name": "arr_u8", "type": ["u8", 18] },
                ]
            }
        },
    }))
    .unwrap();
    // Check that we can use the manual IDL to encode/decode our account in different ways
    let idl_account = idl_program.accounts.get("MyAccount").unwrap();
    let bytes_coordinator_join_run = [
        67, 111, 111, 114, 100, 105, 110, 97, 116, 111, 114, 74, 111, 105, 110,
        82, 117, 110,
    ];
    let case1 = idl_account
        .encode(&json!({
            "bytes": bytes_coordinator_join_run,
            "vec_u8": bytes_coordinator_join_run,
            "arr_u8": bytes_coordinator_join_run,
        }))
        .unwrap();
    let case2 = idl_account
        .encode(&json!({
            "bytes": "CoordinatorJoinRun",
            "vec_u8": "CoordinatorJoinRun",
            "arr_u8": "CoordinatorJoinRun",
        }))
        .unwrap();
    let case3 = idl_account
        .encode(&json!({
            "bytes": {"utf8": "CoordinatorJoinRun"},
            "vec_u8": {"utf8": "CoordinatorJoinRun"},
            "arr_u8": {"utf8": "CoordinatorJoinRun"},
        }))
        .unwrap();
    let case4 = idl_account
        .encode(&json!({
            "bytes": {"base16": "436F6F7264696E61746F724A6F696E52756E"},
            "vec_u8": {"base58": "3oEADzTpGyQHQioFsuM8mzvXf"},
            "arr_u8": {"base64": "Q29vcmRpbmF0b3JKb2luUnVu"},
        }))
        .unwrap();
    let case5 = idl_account
        .encode(&json!({
            "bytes": {
                "value": ["Coordinator", "Join", "Run"],
                "type": ["string"],
            },
            "vec_u8": {
                "value": ["Coordinator", "Join", [82, 117, 110]],
                "type": ["bytes", 3],
            },
            "arr_u8": {
                "value": ["Coordinator", "Join", "Run"],
                "type": {"vec": "string"},
                "prefixed": false,
            },
        }))
        .unwrap();
    // Check that we got the correct results
    let mut expected = vec![];
    expected.extend_from_slice(&idl_account.discriminator);
    expected.extend_from_slice(&18u32.to_le_bytes());
    expected.extend_from_slice(b"CoordinatorJoinRun");
    expected.extend_from_slice(&18u32.to_le_bytes());
    expected.extend_from_slice(b"CoordinatorJoinRun");
    expected.extend_from_slice(b"CoordinatorJoinRun");
    assert_eq!(case1, expected);
    assert_eq!(case2, expected);
    assert_eq!(case3, expected);
    assert_eq!(case4, expected);
    assert_eq!(case5, expected);
}
