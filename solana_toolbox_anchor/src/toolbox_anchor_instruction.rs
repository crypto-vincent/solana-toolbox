use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use anyhow::Result;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_toolbox_endpoint::ToolboxEndpoint;

use crate::toolbox_anchor::ToolboxAnchor;

impl ToolboxAnchor {
    pub fn build_instruction<
        Accounts: ToAccountMetas,
        Args: InstructionData,
    >(
        program_id: Pubkey,
        accounts: Accounts,
        args: Args,
    ) -> Instruction {
        Instruction {
            program_id,
            accounts: accounts.to_account_metas(None),
            data: args.data(),
        }
    }

    pub async fn process_instruction<
        Accounts: ToAccountMetas,
        Args: InstructionData,
    >(
        endpoint: &mut ToolboxEndpoint,
        payer: &Keypair,
        program_id: Pubkey,
        accounts: Accounts,
        args: Args,
    ) -> Result<()> {
        endpoint
            .process_instruction(
                payer,
                ToolboxAnchor::build_instruction(program_id, accounts, args),
            )
            .await?;
        Ok(())
    }

    pub async fn process_instruction_with_signers<
        Accounts: ToAccountMetas,
        Args: InstructionData,
    >(
        endpoint: &mut ToolboxEndpoint,
        payer: &Keypair,
        program_id: Pubkey,
        accounts: Accounts,
        args: Args,
        signers: &[&Keypair],
    ) -> Result<()> {
        endpoint
            .process_instruction_with_signers(
                payer,
                ToolboxAnchor::build_instruction(program_id, accounts, args),
                signers,
            )
            .await?;
        Ok(())
    }
}
