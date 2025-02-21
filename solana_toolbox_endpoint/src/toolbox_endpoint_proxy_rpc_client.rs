use std::collections::HashSet;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use solana_account_decoder::UiDataSliceConfig;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_client::rpc_config::RpcProgramAccountsConfig;
use solana_client::rpc_filter::Memcmp;
use solana_client::rpc_filter::MemcmpEncodedBytes;
use solana_client::rpc_filter::RpcFilterType;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::transaction::Transaction;
use solana_transaction_status::UiReturnDataEncoding;
use solana_transaction_status::UiTransactionEncoding;
use solana_transaction_status::UiTransactionReturnData;

use crate::toolbox_endpoint_data_execution::ToolboxEndpointDataExecution;
use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;
use crate::ToolboxEndpoint;

const WAIT_SLEEP_DURATION: Duration = Duration::from_millis(100);
const WAIT_TIMEOUT_DURATION: Duration = Duration::from_secs(10);

pub struct ToolboxEndpointProxyRpcClient {
    inner: RpcClient,
}

impl ToolboxEndpointProxyRpcClient {
    pub fn new(rpc_client: RpcClient) -> ToolboxEndpointProxyRpcClient {
        ToolboxEndpointProxyRpcClient { inner: rpc_client }
    }
}

#[async_trait::async_trait]
impl ToolboxEndpointProxy for ToolboxEndpointProxyRpcClient {
    async fn get_latest_blockhash(
        &mut self
    ) -> Result<Hash, ToolboxEndpointError> {
        Ok(self.inner.get_latest_blockhash().await?)
    }

    async fn get_balance(
        &mut self,
        address: &Pubkey,
    ) -> Result<u64, ToolboxEndpointError> {
        Ok(self.inner.get_balance(address).await?)
    }

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, ToolboxEndpointError> {
        Ok(self.inner.get_multiple_accounts(addresses).await?)
    }

    async fn simulate_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<ToolboxEndpointDataExecution, ToolboxEndpointError> {
        let outcome = self.inner.simulate_transaction(transaction).await?;
        Ok(ToolboxEndpointDataExecution {
            slot: outcome.context.slot,
            error: outcome.value.err,
            logs: outcome.value.logs,
            return_data: ToolboxEndpointProxyRpcClient::prepare_return_data(
                outcome.value.return_data,
            )?,
            units_consumed: outcome.value.units_consumed,
        })
    }

    async fn process_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<Signature, ToolboxEndpointError> {
        let timer = Instant::now();
        let signature = self.inner.send_transaction(transaction).await?;
        loop {
            if self.inner.confirm_transaction(&signature).await? {
                return Ok(signature);
            }
            if timer.elapsed() > WAIT_TIMEOUT_DURATION {
                return Err(ToolboxEndpointError::Timeout(
                    "Waiting confirmation",
                ));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }

    async fn request_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        Ok(self.inner.request_airdrop(to, lamports).await?)
    }

    async fn get_execution(
        &mut self,
        signature: &Signature,
    ) -> Result<ToolboxEndpointDataExecution, ToolboxEndpointError> {
        let outcome = self
            .inner
            .get_transaction(signature, UiTransactionEncoding::Base64)
            .await?;
        // TODO - shall we get the original transaction here ?
        // let dada = outcome.transaction.transaction.decode();
        match outcome.transaction.meta {
            Some(metadata) => Ok(ToolboxEndpointDataExecution {
                slot: outcome.slot,
                error: metadata.err,
                logs: metadata.log_messages.into(),
                return_data:
                    ToolboxEndpointProxyRpcClient::prepare_return_data(
                        metadata.return_data.into(),
                    )?,
                units_consumed: metadata.compute_units_consumed.into(),
            }),
            None => Err(ToolboxEndpointError::Custom(
                "Unknown transaction execution".to_string(),
            )),
        }
    }

    async fn search_addresses(
        &mut self,
        program_id: &Pubkey,
        data_len: Option<usize>,
        data_chunks: &[(usize, &[u8])],
    ) -> Result<HashSet<Pubkey>, ToolboxEndpointError> {
        let mut program_accounts_filters = vec![];
        if let Some(data_len) = data_len {
            program_accounts_filters.push(RpcFilterType::DataSize(
                u64::try_from(data_len).unwrap(),
            ));
        }
        for (slice_offset, slice_bytes) in data_chunks {
            program_accounts_filters.push(RpcFilterType::Memcmp(Memcmp::new(
                *slice_offset,
                MemcmpEncodedBytes::Base64(STANDARD.encode(slice_bytes)),
            )));
        }
        let program_accounts_config =
            make_program_accounts_config(program_accounts_filters);
        Ok(HashSet::from_iter(
            self.inner
                .get_program_accounts_with_config(
                    program_id,
                    program_accounts_config,
                )
                .await?
                .iter()
                .map(|result| result.0),
        ))
    }

    async fn search_signatures(
        &mut self,
        address: &Pubkey,
        start_before: Option<Signature>,
        rewind_until: Option<Signature>,
        limit: usize,
    ) -> Result<Vec<Signature>, ToolboxEndpointError> {
        let mut oldest_known_signature = start_before;
        let mut ordered_signatures = vec![];
        let mut retries = 0;
        loop {
            let batch_size = match retries {
                0 => 10,
                1 => 100,
                _ => 1000,
            };
            retries += 1;
            let signatures = self
                .inner
                .get_signatures_for_address_with_config(
                    address,
                    GetConfirmedSignaturesForAddress2Config {
                        before: oldest_known_signature,
                        until: None,
                        limit: Some(batch_size),
                        commitment: None,
                    },
                )
                .await?;
            if signatures.is_empty() {
                return Ok(ordered_signatures);
            }
            for signature in &signatures {
                let found_signature = Signature::from_str(&signature.signature)
                    .map_err(ToolboxEndpointError::ParseSignature)?;
                ordered_signatures.push(found_signature);
                if ordered_signatures.len() >= limit {
                    return Ok(ordered_signatures);
                }
                if let Some(rewind_until) = rewind_until {
                    if found_signature == rewind_until {
                        return Ok(ordered_signatures);
                    }
                }
                oldest_known_signature = Some(found_signature);
            }
        }
    }

    async fn forward_clock_unix_timestamp(
        &mut self,
        unix_timestamp_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let timer = Instant::now();
        let unix_timestamp_after =
            self.get_sysvar_clock().await?.unix_timestamp
                + (unix_timestamp_delta as i64);
        loop {
            if self.get_sysvar_clock().await?.unix_timestamp
                >= unix_timestamp_after
            {
                return Ok(());
            }
            if timer.elapsed() > WAIT_TIMEOUT_DURATION {
                return Err(ToolboxEndpointError::Timeout("Clock forwarding"));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }

    async fn forward_clock_slot(
        &mut self,
        slot_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let timer = Instant::now();
        let slot_after = self.get_sysvar_clock().await?.slot + slot_delta;
        loop {
            if self.get_sysvar_clock().await?.slot >= slot_after {
                return Ok(());
            }
            if timer.elapsed() > WAIT_TIMEOUT_DURATION {
                return Err(ToolboxEndpointError::Timeout("Clock forwarding"));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }

    async fn forward_clock_epoch(
        &mut self,
        epoch_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let timer = Instant::now();
        let epoch_after = self.get_sysvar_clock().await?.epoch + epoch_delta;
        loop {
            if self.get_sysvar_clock().await?.epoch >= epoch_after {
                return Ok(());
            }
            if timer.elapsed() > WAIT_TIMEOUT_DURATION {
                return Err(ToolboxEndpointError::Timeout("Clock forwarding"));
            }
            sleep(WAIT_SLEEP_DURATION)
        }
    }
}

impl ToolboxEndpointProxyRpcClient {
    async fn get_sysvar_clock(
        &mut self
    ) -> Result<Clock, ToolboxEndpointError> {
        bincode::deserialize::<Clock>(
            &self
                .inner
                .get_account(&ToolboxEndpoint::SYSVAR_CLOCK_ID)
                .await?
                .data,
        )
        .map_err(ToolboxEndpointError::Bincode)
    }

    fn prepare_return_data(
        return_data: Option<UiTransactionReturnData>
    ) -> Result<Option<Vec<u8>>, ToolboxEndpointError> {
        return_data
            .map(|return_data| {
                let (payload, encoding) = return_data.data;
                if encoding != UiReturnDataEncoding::Base64 {
                    return Err(ToolboxEndpointError::Custom(
                        "Unknown return data encoding".to_string(),
                    ));
                }
                STANDARD
                    .decode(payload)
                    .map_err(ToolboxEndpointError::Base64Decode)
            })
            .transpose()
    }
}

fn make_account_info_config() -> RpcAccountInfoConfig {
    RpcAccountInfoConfig {
        encoding: None,
        data_slice: Some(UiDataSliceConfig { offset: 0, length: 0 }),
        commitment: None,
        min_context_slot: None,
    }
}

#[cfg(not(feature = "has_sort_results_field"))]
fn make_program_accounts_config(
    program_accounts_filters: Vec<RpcFilterType>
) -> RpcProgramAccountsConfig {
    RpcProgramAccountsConfig {
        filters: Some(program_accounts_filters),
        account_config: make_account_info_config(),
        with_context: None,
    }
}

#[cfg(feature = "has_sort_results_field")]
fn make_program_accounts_config(
    program_accounts_filters: Vec<RpcFilterType>
) -> RpcProgramAccountsConfig {
    RpcProgramAccountsConfig {
        filters: Some(program_accounts_filters),
        account_config: make_account_info_config(),
        with_context: None,
        sort_results: None,
    }
}
