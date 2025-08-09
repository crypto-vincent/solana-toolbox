use std::collections::HashMap;

use solana_program_runtime::invoke_context::BuiltinFunctionWithContext;
use solana_program_test::ProgramTest;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;
use crate::toolbox_endpoint_proxy_program_test_context::ToolboxEndpointProxyProgramTestContext;

// TODO - simplify this, we should just need a HashMap<Pubkey, function> or something like that
pub struct ToolboxEndpointProgramTestBuiltinProgram {
    pub id: Pubkey,
    pub name: &'static str,
    pub processor: Option<BuiltinFunctionWithContext>,
}

#[macro_export]
macro_rules! toolbox_endpoint_program_test_builtin_program {
    ($program_name:expr, $program_id:expr, $program_entry:expr) => {
        $crate::ToolboxEndpointProgramTestBuiltinProgram {
            id: $program_id,
            name: $program_name,
            processor: $crate::solana_program_test_processor!($program_entry),
        }
    };
}

#[macro_export]
macro_rules! toolbox_endpoint_program_test_builtin_program_anchor {
    ($program_name:expr, $program_id:expr, $program_entry:expr) => {
        $crate::ToolboxEndpointProgramTestBuiltinProgram {
            id: $program_id,
            name: $program_name,
            processor: $crate::solana_program_test_processor!(
                |program_id, accounts, data| {
                    let accounts = Box::leak(Box::new(accounts.to_vec()));
                    $program_entry(program_id, accounts, data)
                }
            ),
        }
    };
}

impl ToolboxEndpoint {
    pub async fn new_program_test() -> ToolboxEndpoint {
        ToolboxEndpoint::new_program_test_with_config(
            &[],
            HashMap::default(),
            HashMap::default(),
        )
        .await
    }

    pub async fn new_program_test_with_builtin_programs(
        builtin_programs: &[ToolboxEndpointProgramTestBuiltinProgram],
    ) -> ToolboxEndpoint {
        ToolboxEndpoint::new_program_test_with_config(
            builtin_programs,
            HashMap::default(),
            HashMap::default(),
        )
        .await
    }

    pub async fn new_program_test_with_preloaded_programs(
        preloaded_programs: HashMap<Pubkey, &'static str>,
    ) -> ToolboxEndpoint {
        ToolboxEndpoint::new_program_test_with_config(
            &[],
            preloaded_programs,
            HashMap::default(),
        )
        .await
    }

    pub async fn new_program_test_with_preloaded_programs_and_accounts(
        preloaded_programs: HashMap<Pubkey, &'static str>,
        preloaded_accounts: HashMap<Pubkey, Account>,
    ) -> ToolboxEndpoint {
        ToolboxEndpoint::new_program_test_with_config(
            &[],
            preloaded_programs,
            preloaded_accounts,
        )
        .await
    }

    pub async fn new_program_test_with_config(
        builtin_programs: &[ToolboxEndpointProgramTestBuiltinProgram],
        preloaded_programs: HashMap<Pubkey, &'static str>,
        preloaded_accounts: HashMap<Pubkey, Account>,
    ) -> ToolboxEndpoint {
        let mut program_test = ProgramTest::default();
        for builtin_program in builtin_programs {
            program_test.add_program(
                builtin_program.name,
                builtin_program.id,
                builtin_program.processor,
            );
        }
        program_test.prefer_bpf(true);
        for (preloaded_program_id, preloaded_program_path) in preloaded_programs
        {
            program_test.add_program(
                preloaded_program_path,
                preloaded_program_id,
                None,
            );
        }
        for (preloaded_account_address, preloaded_account) in preloaded_accounts
        {
            program_test
                .add_account(preloaded_account_address, preloaded_account);
        }
        let mut proxy_program_test_context =
            ToolboxEndpointProxyProgramTestContext::new(
                program_test.start_with_context().await,
            );
        proxy_program_test_context.save_slot_unix_timestamp().await;
        let proxy: Box<dyn ToolboxEndpointProxy> =
            Box::new(proxy_program_test_context);
        ToolboxEndpoint::from(proxy)
    }
}
