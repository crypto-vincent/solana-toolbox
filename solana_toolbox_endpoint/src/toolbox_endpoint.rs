use std::collections::HashSet;

use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint_data_execution::ToolboxEndpointDataExecution;
use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_logger::ToolboxEndpointLogger;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

pub struct ToolboxEndpoint {
    proxy: Box<dyn ToolboxEndpointProxy>,
    loggers: Vec<Box<dyn ToolboxEndpointLogger>>,
}

impl From<Box<dyn ToolboxEndpointProxy>> for ToolboxEndpoint {
    fn from(proxy: Box<dyn ToolboxEndpointProxy>) -> Self {
        Self { proxy, loggers: vec![] }
    }
}

impl ToolboxEndpoint {
    pub fn add_logger(
        &mut self,
        logger: Box<dyn ToolboxEndpointLogger>,
    ) {
        self.loggers.push(logger);
    }

    pub async fn get_latest_blockhash(
        &mut self
    ) -> Result<Hash, ToolboxEndpointError> {
        self.proxy.get_latest_blockhash().await
    }

    pub async fn get_balance(
        &mut self,
        address: &Pubkey,
    ) -> Result<u64, ToolboxEndpointError> {
        self.proxy.get_balance(address).await
    }

    pub async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, ToolboxEndpointError> {
        self.proxy.get_accounts(addresses).await
    }

    pub async fn simulate_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<ToolboxEndpointDataExecution, ToolboxEndpointError> {
        self.proxy.simulate_transaction(transaction).await
    }

    pub async fn process_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<Signature, ToolboxEndpointError> {
        let result = self.proxy.process_transaction(transaction).await;
        for logger in &self.loggers {
            logger.on_transaction(transaction, &result).await;
        }
        result
    }

    pub async fn request_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        self.proxy.request_airdrop(to, lamports).await
    }

    pub async fn get_execution(
        &mut self,
        signature: &Signature,
    ) -> Result<ToolboxEndpointDataExecution, ToolboxEndpointError> {
        self.proxy.get_execution(signature).await
    }

    pub async fn search_addresses(
        &mut self,
        program_id: &Pubkey,
        data_len: Option<usize>,
        data_chunks: &[(usize, &[u8])],
    ) -> Result<HashSet<Pubkey>, ToolboxEndpointError> {
        self.proxy.search_addresses(program_id, data_len, data_chunks).await
    }

    pub async fn search_signatures(
        &mut self,
        address: &Pubkey,
        start_before: Option<Signature>,
        rewind_until: Option<Signature>,
        limit: usize,
    ) -> Result<Vec<Signature>, ToolboxEndpointError> {
        self.proxy
            .search_signatures(address, start_before, rewind_until, limit)
            .await
    }

    pub async fn forward_clock_unix_timestamp(
        &mut self,
        unix_timestamp_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        self.proxy.forward_clock_unix_timestamp(unix_timestamp_delta).await
    }

    pub async fn forward_clock_slot(
        &mut self,
        slot_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        self.proxy.forward_clock_slot(slot_delta).await
    }

    pub async fn forward_clock_epoch(
        &mut self,
        epoch_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        self.proxy.forward_clock_epoch(epoch_delta).await
    }
}
