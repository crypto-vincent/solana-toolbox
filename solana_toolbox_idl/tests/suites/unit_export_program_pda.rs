use serde_json::json;
use solana_toolbox_idl::ToolboxIdlFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
                "discriminator": [],
                "accounts": [
                    { "name": "info" },
                    {
                        "name": "addr",
                        "pda": {
                            "seeds": [
                                [1, 2, 3],
                                {
                                    "kind": "arg",
                                    "path": "param.field",
                                },
                                {
                                    "path": "info.struct.field",
                                    "account": "MyAccount",
                                },
                            ]
                        }
                    }
                ],
                "args": [
                    {"name": "param", "type": "MyStruct"},
                ]
            }
        },
        "accounts": {
            "MyAccount": {
                "discriminator": [],
                "fields": [
                    { "name": "struct", "type": "MyStruct" }
                ],
            },
        },
        "types": {
            "MyStruct": {
                "fields": [
                    { "name": "field", "type": "u8" },
                ],
            },
        },
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::human()),
        json!({
            "metadata": {},
            "instructions": {
                "my_ix": {
                    "discriminator": [],
                    "accounts": [
                        { "name": "info" },
                        {
                            "name": "addr",
                            "pda": {
                                "seeds": [
                                    [1, 2, 3],
                                    {
                                        "kind": "arg",
                                        "path": "param.field",
                                    },
                                    {
                                        "path": "info.struct.field",
                                        "account": "MyAccount",
                                    },
                                ]
                            }
                        }
                    ],
                    "args": [
                        {"name": "param", "type": "MyStruct"},
                    ]
                }
            },
            "accounts": {
                "MyAccount": {
                    "discriminator": [],
                    "type": {
                        "fields": [
                            { "name": "struct", "type": "MyStruct" }
                        ],
                    },
                },
            },
            "types": {
                "MyStruct": {
                    "fields": [
                        { "name": "field", "type": "u8" },
                    ],
                },
            },
            "errors": {},
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::anchor_26()),
        json!({
            "instructions": [
                {
                    "name": "my_ix",
                    "discriminator": [],
                    "accounts": [
                        { "name": "info" },
                        {
                            "name": "addr",
                            "pda": {
                                "seeds": [
                                    {
                                        "kind": "const",
                                        "type": "bytes",
                                        "value": [1, 2, 3],
                                    },
                                    {
                                        "kind": "arg",
                                        "type": "u8",
                                        "path": "param.field",
                                    },
                                    {
                                        "kind": "account",
                                        "type": "u8",
                                        "path": "info.struct.field",
                                        "account": "MyAccount",
                                    },
                                ]
                            }
                        }
                    ],
                    "args": [
                        {
                            "name": "param",
                            "type": { "defined": "MyStruct" },
                        },
                    ]
                }
            ],
            "accounts": [
                {
                    "name": "MyAccount",
                    "discriminator": [],
                    "type": {
                        "kind": "struct",
                        "fields": [
                            {
                                "name": "struct",
                                "type": { "defined": "MyStruct" },
                            }
                        ],
                    }
                }
            ],
            "types": [
                {
                    "name": "MyStruct",
                    "type": {
                        "kind": "struct",
                        "fields": [
                            { "name": "field", "type": "u8" },
                        ],
                    }
                }
            ],
            "errors": [],
        })
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::anchor_30()),
        json!({
            "metadata": {},
            "instructions": [
                {
                    "name": "my_ix",
                    "discriminator": [],
                    "accounts": [
                        { "name": "info" },
                        {
                            "name": "addr",
                            "pda": {
                                "seeds": [
                                    {
                                        "kind": "const",
                                        "type": "bytes",
                                        "value": [1, 2, 3],
                                    },
                                    {
                                        "kind": "arg",
                                        "type": "u8",
                                        "path": "param.field",
                                    },
                                    {
                                        "kind": "account",
                                        "type": "u8",
                                        "path": "info.struct.field",
                                        "account": "MyAccount",
                                    },
                                ]
                            }
                        }
                    ],
                    "args": [
                        {
                            "name": "param",
                            "type": {"defined": {"name": "MyStruct"}}
                        },
                    ]
                }
            ],
            "accounts": [
                {
                    "name": "MyAccount",
                    "discriminator": [],
                    "type": {
                        "kind": "struct",
                        "fields": [
                            {
                                "name": "struct",
                                "type": {"defined": {"name": "MyStruct"}}
                            }
                        ],
                    }
                }
            ],
            "types": [
                {
                    "name": "MyStruct",
                    "type": {
                        "kind": "struct",
                        "fields": [
                            { "name": "field", "type": "u8" },
                        ],
                    }
                }
            ],
            "errors": [],
        })
    );
}
