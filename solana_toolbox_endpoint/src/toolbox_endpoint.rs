use std::collections::HashSet;

use anyhow::Result;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;
use solana_sdk::transaction::VersionedTransaction;

use crate::toolbox_endpoint_execution::ToolboxEndpointExecution;
use crate::toolbox_endpoint_logger::ToolboxEndpointLogger;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;

pub struct ToolboxEndpoint {
    proxy: Box<dyn ToolboxEndpointProxy>,
    loggers: Vec<Box<dyn ToolboxEndpointLogger>>,
}

impl From<Box<dyn ToolboxEndpointProxy>> for ToolboxEndpoint {
    fn from(proxy: Box<dyn ToolboxEndpointProxy>) -> ToolboxEndpoint {
        ToolboxEndpoint {
            proxy,
            loggers: vec![],
        }
    }
}

impl ToolboxEndpoint {
    pub fn add_logger(&mut self, logger: Box<dyn ToolboxEndpointLogger>) {
        self.loggers.push(logger);
    }

    pub async fn get_latest_blockhash(&mut self) -> Result<Hash> {
        self.proxy.get_latest_blockhash().await
    }

    pub async fn get_slot_unix_timestamp(&mut self, slot: u64) -> Result<i64> {
        self.proxy.get_slot_unix_timestamp(slot).await
    }

    pub async fn get_balance(&mut self, address: &Pubkey) -> Result<u64> {
        self.proxy.get_balance(address).await
    }

    pub async fn get_account(
        &mut self,
        address: &Pubkey,
    ) -> Result<Option<Account>> {
        self.proxy.get_account(address).await
    }

    pub async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>> {
        self.proxy.get_accounts(addresses).await
    }

    pub async fn simulate_transaction(
        &mut self,
        transaction: Transaction,
        verify_signatures: bool,
    ) -> Result<ToolboxEndpointExecution> {
        self.simulate_versioned_transaction(
            transaction.into(),
            verify_signatures,
        )
        .await
    }

    pub async fn simulate_versioned_transaction(
        &mut self,
        versioned_transaction: VersionedTransaction,
        verify_signatures: bool,
    ) -> Result<ToolboxEndpointExecution> {
        self.proxy
            .simulate_transaction(versioned_transaction, verify_signatures)
            .await
    }

    pub async fn process_transaction(
        &mut self,
        transaction: Transaction,
        verify_prelight: bool,
    ) -> Result<(Signature, ToolboxEndpointExecution)> {
        self.process_versioned_transaction(transaction.into(), verify_prelight)
            .await
    }

    pub async fn process_versioned_transaction(
        &mut self,
        versioned_transaction: VersionedTransaction,
        verify_prelight: bool,
    ) -> Result<(Signature, ToolboxEndpointExecution)> {
        let processed = self
            .proxy
            .process_transaction(versioned_transaction, verify_prelight)
            .await?;
        for logger in &self.loggers {
            logger.on_processed(&processed).await;
        }
        Ok(processed)
    }

    pub async fn request_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<(Signature, ToolboxEndpointExecution)> {
        let processed = self.proxy.request_airdrop(to, lamports).await?;
        for logger in &self.loggers {
            logger.on_processed(&processed).await;
        }
        Ok(processed)
    }

    pub async fn get_execution(
        &mut self,
        signature: &Signature,
    ) -> Result<ToolboxEndpointExecution> {
        self.proxy.get_execution(signature).await
    }

    pub async fn search_addresses(
        &mut self,
        program_id: &Pubkey,
        data_len: Option<usize>,
        data_chunks: &[(usize, &[u8])],
    ) -> Result<HashSet<Pubkey>> {
        self.proxy
            .search_addresses(program_id, data_len, data_chunks)
            .await
    }

    pub async fn search_signatures(
        &mut self,
        address: &Pubkey,
        limit: usize,
        start_before: Option<Signature>,
        rewind_until: Option<Signature>,
    ) -> Result<Vec<Signature>> {
        self.proxy
            .search_signatures(address, limit, start_before, rewind_until)
            .await
    }

    pub async fn forward_clock_unix_timestamp(
        &mut self,
        unix_timestamp_delta: u64,
    ) -> Result<()> {
        self.proxy
            .forward_clock_unix_timestamp(unix_timestamp_delta)
            .await
    }

    pub async fn forward_clock_slot(&mut self, slot_delta: u64) -> Result<()> {
        self.proxy.forward_clock_slot(slot_delta).await
    }

    pub async fn forward_clock_epoch(
        &mut self,
        epoch_delta: u64,
    ) -> Result<()> {
        self.proxy.forward_clock_epoch(epoch_delta).await
    }
}
