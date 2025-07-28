use std::collections::HashMap;
use std::sync::LazyLock;

use anyhow::Context;
use anyhow::Result;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_idl_program::ToolboxIdlProgram;

static KNOWN_IDLS: LazyLock<HashMap<Pubkey, &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        (
            ToolboxEndpoint::SYSTEM_PROGRAM_ID,
            include_str!("lib/native_system.json"),
        ),
        (
            ToolboxEndpoint::ADDRESS_LOOKUP_TABLE_PROGRAM_ID,
            include_str!("lib/native_address_lookup_table.json"),
        ),
        (
            ToolboxEndpoint::COMPUTE_BUDGET_PROGRAM_ID,
            include_str!("lib/native_compute_budget.json"),
        ),
        (
            ToolboxEndpoint::NATIVE_LOADER_PROGRAM_ID,
            include_str!("lib/native_loader.json"),
        ),
        (
            ToolboxEndpoint::BPF_LOADER_2_PROGRAM_ID,
            include_str!("lib/native_bpf_loader_2.json"),
        ),
        (
            ToolboxEndpoint::BPF_LOADER_UPGRADEABLE_PROGRAM_ID,
            include_str!("lib/native_bpf_loader_upgradeable.json"),
        ),
        (
            ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            include_str!("lib/spl_token.json"),
        ),
        (
            ToolboxEndpoint::SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
            include_str!("lib/spl_associated_token.json"),
        ),
        (
            pubkey!("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX"),
            include_str!("lib/spl_name_service.json"),
        ),
        (
            pubkey!("L2TExMFKdjpN9kozasaurPirfHy9P8sbXoAN1qA3S95"),
            include_str!("lib/misc_lighthouse.json"),
        ),
    ])
});

impl ToolboxIdlProgram {
    pub fn from_lib(program_id: &Pubkey) -> Option<ToolboxIdlProgram> {
        KNOWN_IDLS.get(program_id).map(|content| {
            ToolboxIdlProgram::try_parse_from_str(content)
                .context(program_id.to_string())
                .unwrap()
        })
    }

    pub fn find_anchor_address(program_id: &Pubkey) -> Result<Pubkey> {
        let base = Pubkey::find_program_address(&[], program_id).0;
        Ok(Pubkey::create_with_seed(&base, "anchor:idl", program_id)?)
    }
}
