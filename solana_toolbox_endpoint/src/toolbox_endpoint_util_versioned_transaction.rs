use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::anyhow;
use anyhow::Result;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::v0::Message;
use solana_sdk::message::CompileError;
use solana_sdk::message::VersionedMessage;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::VersionedTransaction;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub fn compile_versioned_transaction(
        payer: &Keypair,
        instructions: &[Instruction],
        signers: &[&Keypair],
        resolved_address_lookup_tables: &[(Pubkey, Vec<Pubkey>)],
        recent_blockhash: Hash,
    ) -> Result<VersionedTransaction> {
        let mut needed_signers_pubkeys = HashSet::new();
        for instruction in instructions {
            for instruction_account in &instruction.accounts {
                if instruction_account.is_signer {
                    needed_signers_pubkeys.insert(instruction_account.pubkey);
                }
            }
        }
        let mut signers_pubkeys = HashSet::new();
        let mut signers_keypairs = vec![];
        signers_pubkeys.insert(payer.pubkey());
        signers_keypairs.push(payer);
        for signer_keypair in signers {
            let signer_pubkey = signer_keypair.pubkey();
            if !needed_signers_pubkeys.contains(&signer_pubkey) {
                continue;
            }
            if !signers_pubkeys.contains(&signer_pubkey) {
                signers_pubkeys.insert(signer_pubkey);
                signers_keypairs.push(signer_keypair);
            }
        }
        let versioned_transaction = VersionedTransaction::try_new(
            ToolboxEndpoint::compile_versioned_transaction_message(
                &payer.pubkey(),
                instructions,
                resolved_address_lookup_tables,
                recent_blockhash,
            )?,
            &signers_keypairs,
        )?;
        Ok(versioned_transaction)
    }

    pub fn compile_versioned_transaction_message(
        payer: &Pubkey,
        instructions: &[Instruction],
        resolved_address_lookup_tables: &[(Pubkey, Vec<Pubkey>)],
        recent_blockhash: Hash,
    ) -> Result<VersionedMessage> {
        let mut address_lookup_table_accounts = vec![];
        for resolved_address_lookup_table in resolved_address_lookup_tables {
            address_lookup_table_accounts.push(AddressLookupTableAccount {
                key: resolved_address_lookup_table.0,
                addresses: resolved_address_lookup_table.1.to_vec(),
            });
        }
        Ok(VersionedMessage::V0(Message::try_compile(
            payer,
            instructions,
            &address_lookup_table_accounts,
            recent_blockhash,
        )?))
    }

    pub fn decompile_versioned_transaction(
        versioned_transaction: &VersionedTransaction,
        resolved_address_lookup_tables: &[(Pubkey, Vec<Pubkey>)],
    ) -> Result<(Pubkey, Vec<Instruction>)> {
        let mut resolved_address_lookup_tables_addresses = HashMap::new();
        for resolved_address_lookup_table in resolved_address_lookup_tables {
            resolved_address_lookup_tables_addresses.insert(
                resolved_address_lookup_table.0,
                &resolved_address_lookup_table.1[..],
            );
        }
        let mut loaded_writable_addresses = vec![];
        let mut loaded_readonly_addresses = vec![];
        if let Some(message_address_table_lookups) =
            versioned_transaction.message.address_table_lookups()
        {
            for message_address_table_lookup in message_address_table_lookups {
                let resolved_address_lookup_table_addresses =
                    resolved_address_lookup_tables_addresses
                        .get(&message_address_table_lookup.account_key)
                        .ok_or(CompileError::AddressTableLookupIndexOverflow)?;
                for message_address_table_lookup_writable_index in
                    &message_address_table_lookup.writable_indexes
                {
                    loaded_writable_addresses.push(
                        *resolved_address_lookup_table_addresses
                            .get(usize::from(
                                *message_address_table_lookup_writable_index,
                            ))
                            .ok_or(
                                CompileError::AddressTableLookupIndexOverflow,
                            )?,
                    );
                }
                for message_address_table_lookup_readonly_index in
                    &message_address_table_lookup.readonly_indexes
                {
                    loaded_readonly_addresses.push(
                        *resolved_address_lookup_table_addresses
                            .get(usize::from(
                                *message_address_table_lookup_readonly_index,
                            ))
                            .ok_or(
                                CompileError::AddressTableLookupIndexOverflow,
                            )?,
                    );
                }
            }
        }
        ToolboxEndpoint::decompile_versioned_transaction_with_loaded_addresses(
            versioned_transaction,
            &loaded_writable_addresses,
            &loaded_readonly_addresses,
        )
    }

    pub fn decompile_versioned_transaction_with_loaded_addresses(
        versioned_transaction: &VersionedTransaction,
        loaded_writable_addresses: &[Pubkey],
        loaded_readonly_addresses: &[Pubkey],
    ) -> Result<(Pubkey, Vec<Instruction>)> {
        let header = versioned_transaction.message.header();
        let static_addresses =
            versioned_transaction.message.static_account_keys();
        let payer =
            ToolboxEndpoint::decompile_transaction_payer(static_addresses)?;
        let instructions = ToolboxEndpoint::decompile_transaction_instructions(
            header.num_required_signatures,
            header.num_readonly_signed_accounts,
            header.num_readonly_unsigned_accounts,
            static_addresses,
            loaded_writable_addresses,
            loaded_readonly_addresses,
            versioned_transaction.message.instructions(),
        )?;
        Ok((payer, instructions))
    }

    pub fn verify_versioned_transaction_length(
        versioned_transaction: &VersionedTransaction,
    ) -> Result<()> {
        let transaction_length =
            bincode::serialize(versioned_transaction)?.len();
        let limit_length = ToolboxEndpoint::TRANSACTION_LENGTH_LIMIT;
        if transaction_length > limit_length {
            return Err(anyhow!(
                "VersionedTransaction of size {} exceeds the limit of {} bytes",
                transaction_length,
                limit_length
            ));
        }
        Ok(())
    }

    pub fn verify_versioned_transaction_signatures(
        versioned_transaction: &VersionedTransaction,
    ) -> Result<()> {
        let verified_signatures = versioned_transaction.verify_with_results();
        let found_signatures = verified_signatures.len();
        let expected_signatures = usize::from(
            versioned_transaction
                .message
                .header()
                .num_required_signatures,
        );
        if found_signatures != expected_signatures {
            return Err(anyhow!(
                "VersionedTransaction has {} signatures, but requires {}",
                found_signatures,
                expected_signatures
            ));
        }
        if !verified_signatures
            .iter()
            .all(|verify_result| *verify_result)
        {
            return Err(anyhow!("VersionedTransaction signatures are invalid"));
        }
        Ok(())
    }
}
