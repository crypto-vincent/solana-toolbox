use serde_json::json;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse(&json!({
        "accounts": {
            "MyAccount": {
                "fields": [
                    { "name": "value", "type": "string" },
                ]
            }
        },
    }))
    .unwrap();
    // Various string cases to test
    let long_string = "Long string ".repeat(1000);
    let tests = vec![
        "Hello world",
        "",
        "Special chars !@#$%^&*()_+-=[]{}|;':\",.<>/?`~",
        "Unicode: 測試, тест, اختبار, परीक्षण",
        &long_string,
        "Emoji: 😀🚀🌟🔥💧🍕🎉",
        "Multiline:\nLine 1\nLine 2\nLine 3",
        "Whitespace:    \t   \n  ",
        "Control chars: \x00\x01\x02\x03\x04\x05\x06\x07\x08\x09",
        "Mixed: Hello, 世界! 12345 🚀🔥\nNew line and \t tab.",
    ];
    // Check that we can properly serialize various strings
    let idl_account = idl_program.accounts.get("MyAccount").unwrap();
    for test in tests {
        let value = json!({ "value": test });
        let encoded = idl_account.encode(&value).unwrap();
        let decoded = idl_account.decode(&encoded).unwrap();
        assert_eq!(value, decoded);
    }
}
