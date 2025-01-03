use solana_program_test::ProgramTestBanksClientExt;
use solana_program_test::ProgramTestContext;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::transfer;
use solana_sdk::sysvar::clock::Clock;
use solana_sdk::transaction::Transaction;

use crate::toolbox_endpoint_error::ToolboxEndpointError;
use crate::toolbox_endpoint_inner::ToolboxEndpointInner;

#[async_trait::async_trait]
impl ToolboxEndpointInner for ProgramTestContext {
    async fn get_latest_blockhash(
        &mut self
    ) -> Result<Hash, ToolboxEndpointError> {
        Ok(self.last_blockhash)
    }

    async fn get_accounts(
        &mut self,
        addresses: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, ToolboxEndpointError> {
        let mut accounts = vec![];
        for address in addresses {
            accounts.push(
                self.banks_client
                    .get_account(*address)
                    .await
                    .map_err(ToolboxEndpointError::BanksClient)?,
            )
        }
        Ok(accounts)
    }

    async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<Signature, ToolboxEndpointError> {
        self.last_blockhash = self
            .banks_client
            .get_new_latest_blockhash(&self.last_blockhash)
            .await
            .map_err(ToolboxEndpointError::Io)?;
        self.banks_client
            .process_transaction(transaction)
            .await
            .map_err(ToolboxEndpointError::BanksClient)?;
        Ok(Signature::default())
    }

    async fn process_airdrop(
        &mut self,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<Signature, ToolboxEndpointError> {
        let instruction = transfer(&self.payer.pubkey(), to, lamports);
        let latest_blockhash = self.get_latest_blockhash().await?;
        let mut transaction: Transaction = Transaction::new_with_payer(
            &[instruction.clone()],
            Some(&self.payer.pubkey()),
        );
        transaction.partial_sign(&[&self.payer], latest_blockhash);
        self.process_transaction(transaction).await
    }

    async fn move_clock_forward(
        &mut self,
        unix_timestamp_delta: u64,
        slot_delta: u64,
    ) -> Result<(), ToolboxEndpointError> {
        let current_clock = self
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .map_err(ToolboxEndpointError::BanksClient)?;
        let mut forwarded_clock = current_clock;
        forwarded_clock.epoch += 1;
        forwarded_clock.slot += slot_delta;
        forwarded_clock.unix_timestamp +=
            i64::try_from(unix_timestamp_delta).unwrap();
        self.set_sysvar::<Clock>(&forwarded_clock);
        Ok(())
    }
}
