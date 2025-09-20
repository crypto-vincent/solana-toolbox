use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_instruction_blob::ToolboxIdlInstructionBlob;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlInstructionAccount {
    pub name: String,
    pub docs: Option<Value>,
    pub writable: bool,
    pub signer: bool,
    pub optional: bool,
    pub address: Option<Pubkey>,
    pub pda: Option<ToolboxIdlInstructionAccountPda>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlInstructionAccountPda {
    pub seeds: Vec<ToolboxIdlInstructionBlob>,
    pub program: Option<ToolboxIdlInstructionBlob>,
}
