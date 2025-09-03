use std::fs::exists;
use std::fs::read_to_string;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use serde_json::json;
use serde_json::Value;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlInstruction;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlService;

use crate::toolbox_cli_key::ToolboxCliKey;

pub struct ToolboxCliContext {
    json_rpc_url: String,
    commitment: String,
    keypair_path: String,
    custom_idls: Vec<String>,
}

impl ToolboxCliContext {
    pub fn new(
        json_rpc_url: String,
        commitment: String,
        keypair_path: String,
        custom_idls: Vec<String>,
    ) -> ToolboxCliContext {
        ToolboxCliContext {
            json_rpc_url,
            commitment,
            keypair_path,
            custom_idls,
        }
    }

    pub async fn create_endpoint(&self) -> Result<ToolboxEndpoint> {
        Ok(ToolboxEndpoint::new_rpc_with_url_or_moniker_and_commitment(
            &self.json_rpc_url,
            CommitmentConfig::from_str(&self.commitment)?,
        ))
    }

    pub async fn create_service(&self) -> Result<ToolboxIdlService> {
        let mut idl_service = ToolboxIdlService::new();
        for custom_idl in &self.custom_idls {
            if let Some((program_id, idl_file)) = custom_idl.split_once(":") {
                idl_service.set_program(
                    &self.parse_key(program_id)?.address(),
                    Some(
                        ToolboxIdlProgram::try_parse_from_str(
                            &read_to_string(idl_file)?,
                        )?
                        .into(),
                    ),
                );
            } else {
                return Err(anyhow!(
                    "Invalid idl, expected format: [ProgramId:IdlFilePath]"
                ));
            }
        }
        Ok(idl_service)
    }

    pub fn parse_account(
        &self,
        account: &str,
    ) -> Result<(String, ToolboxCliKey)> {
        if let Some((name, key)) = account.split_once(":") {
            return Ok((name.to_string(), self.parse_key(key)?));
        }
        // TODO - use custom clap parser instead
        Err(anyhow!(
            "Invalid account, expected format: [Name:[Pubkey|KeypairFilePath]]"
        ))
    }

    pub fn parse_key(&self, key: &str) -> Result<ToolboxCliKey> {
        if key.eq_ignore_ascii_case("keypair")
            || key.eq_ignore_ascii_case("wallet")
            || key.eq_ignore_ascii_case("config")
        {
            return Ok(ToolboxCliKey::Keypair(self.get_keypair()?));
        }
        if exists(key)? {
            return Ok(ToolboxCliKey::Keypair(self.load_keypair(key)?));
        }
        if let Ok(keypair) =
            ToolboxEndpoint::sanitize_and_decode_keypair_base58(key)
        {
            return Ok(ToolboxCliKey::Keypair(keypair));
        }
        if let Ok(keypair) =
            ToolboxEndpoint::sanitize_and_decode_keypair_json_array(key)
        {
            return Ok(ToolboxCliKey::Keypair(keypair));
        }
        Ok(ToolboxCliKey::Pubkey(
            ToolboxEndpoint::sanitize_and_decode_pubkey(key)?,
        ))
    }

    pub fn parse_signature(&self, value: &str) -> Result<Signature> {
        Ok(Signature::from_str(value)?)
    }

    pub fn parse_hjson(&self, value: &str) -> Result<Value> {
        Ok(serde_hjson::from_str::<Value>(value)?)
    }

    pub fn get_keypair(&self) -> Result<Keypair> {
        self.load_keypair(&self.keypair_path)
    }

    pub fn load_keypair(&self, path: &str) -> Result<Keypair> {
        read_keypair_file(path).map_err(|error| {
            anyhow!("Failed to read keypair file {}: {}", path, error)
        })
    }

    pub fn compute_error_json(&self, error: Error) -> Value {
        json!(error
            .chain()
            .map(|error| error.to_string())
            .collect::<Vec<_>>())
    }

    pub fn compute_account_name(
        &self,
        program: &ToolboxIdlProgram,
        account: &ToolboxIdlAccount,
    ) -> String {
        if let Some(program_name) = &program.metadata.name {
            return format!("{}.{}", program_name, account.name);
        }
        format!("?.{}", account.name)
    }

    pub fn compute_instruction_name(
        &self,
        program: &ToolboxIdlProgram,
        instruction: &ToolboxIdlInstruction,
    ) -> String {
        if let Some(program_name) = &program.metadata.name {
            return format!("{}.{}", program_name, instruction.name);
        }
        format!("?.{}", instruction.name)
    }

    pub fn compute_explorer_address_url(&self, address: &Pubkey) -> String {
        ToolboxEndpoint::compute_explorer_address_url(
            &self.json_rpc_url,
            address,
        )
    }

    pub fn compute_explorer_signature_url(
        &self,
        signature: &Signature,
    ) -> String {
        ToolboxEndpoint::compute_explorer_signature_url(
            &self.json_rpc_url,
            signature,
        )
    }

    pub fn compute_explorer_simulation_url(
        &self,
        transaction_signatures: &[Signature],
        transaction_message_serialized: &[u8],
    ) -> String {
        ToolboxEndpoint::compute_explorer_simulation_url(
            &self.json_rpc_url,
            transaction_signatures,
            transaction_message_serialized,
        )
    }
}
