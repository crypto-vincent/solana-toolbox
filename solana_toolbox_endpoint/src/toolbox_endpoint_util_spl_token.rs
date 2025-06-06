use anyhow::Result;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::create_account;
use spl_token::instruction::burn;
use spl_token::instruction::freeze_account;
use spl_token::instruction::initialize_account;
use spl_token::instruction::initialize_mint;
use spl_token::instruction::mint_to;
use spl_token::instruction::set_authority;
use spl_token::instruction::thaw_account;
use spl_token::instruction::transfer;
use spl_token::instruction::AuthorityType;
use spl_token::native_mint;
use spl_token::state::Account;
use spl_token::state::Mint;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub const SPL_TOKEN_NATIVE_MINT: Pubkey = native_mint::ID;
    pub const SPL_TOKEN_PROGRAM_ID: Pubkey = spl_token::ID;

    pub async fn process_spl_token_mint_new(
        &mut self,
        payer: &Keypair,
        mint_authority: &Pubkey,
        mint_freeze_authority: Option<&Pubkey>,
        mint_decimals: u8,
    ) -> Result<Pubkey> {
        let mint = Keypair::new();
        self.process_spl_token_mint_init(
            payer,
            &mint,
            mint_authority,
            mint_freeze_authority,
            mint_decimals,
        )
        .await?;
        Ok(mint.pubkey())
    }

    pub async fn process_spl_token_mint_init(
        &mut self,
        payer: &Keypair,
        mint: &Keypair,
        mint_authority: &Pubkey,
        mint_freeze_authority: Option<&Pubkey>,
        mint_decimals: u8,
    ) -> Result<()> {
        let rent_space = Mint::LEN;
        let rent_minimum_lamports =
            self.get_sysvar_rent().await?.minimum_balance(rent_space);
        let instruction_create = create_account(
            &payer.pubkey(),
            &mint.pubkey(),
            rent_minimum_lamports,
            u64::try_from(rent_space)?,
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
        );
        let instruction_init = initialize_mint(
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            &mint.pubkey(),
            mint_authority,
            mint_freeze_authority,
            mint_decimals,
        )?;
        self.process_instructions_with_signers(
            payer,
            &[instruction_create, instruction_init],
            &[mint],
        )
        .await?;
        Ok(())
    }

    pub async fn process_spl_token_mint_set_authority(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        source_mint_authority: &Keypair,
        destination_mint_authority: Option<&Pubkey>,
    ) -> Result<()> {
        let instruction = set_authority(
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            mint,
            destination_mint_authority,
            AuthorityType::MintTokens,
            &source_mint_authority.pubkey(),
            &[],
        )?;
        self.process_instruction_with_signers(
            payer,
            instruction,
            &[source_mint_authority],
        )
        .await?;
        Ok(())
    }

    pub async fn process_spl_token_mint_to(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        mint_authority: &Keypair,
        destination_token_account: &Pubkey,
        amount: u64,
    ) -> Result<()> {
        let instruction = mint_to(
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            mint,
            destination_token_account,
            &mint_authority.pubkey(),
            &[],
            amount,
        )?;
        self.process_instruction_with_signers(
            payer,
            instruction,
            &[mint_authority],
        )
        .await?;
        Ok(())
    }

    pub async fn process_spl_token_mint_set_freeze_authority(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        source_mint_freeze_authority: &Keypair,
        destination_mint_freeze_authority: Option<&Pubkey>,
    ) -> Result<()> {
        let instruction = set_authority(
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            mint,
            destination_mint_freeze_authority,
            AuthorityType::FreezeAccount,
            &source_mint_freeze_authority.pubkey(),
            &[],
        )?;
        self.process_instruction_with_signers(
            payer,
            instruction,
            &[source_mint_freeze_authority],
        )
        .await?;
        Ok(())
    }

    pub async fn process_spl_token_freeze(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        mint_freeze_authority: &Keypair,
        token_account: &Pubkey,
    ) -> Result<()> {
        let instruction = freeze_account(
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            token_account,
            mint,
            &mint_freeze_authority.pubkey(),
            &[],
        )?;
        self.process_instruction_with_signers(
            payer,
            instruction,
            &[mint_freeze_authority],
        )
        .await?;
        Ok(())
    }

    pub async fn process_spl_token_thaw(
        &mut self,
        payer: &Keypair,
        mint: &Pubkey,
        mint_freeze_authority: &Keypair,
        token_account: &Pubkey,
    ) -> Result<()> {
        let instruction = thaw_account(
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            token_account,
            mint,
            &mint_freeze_authority.pubkey(),
            &[],
        )?;
        self.process_instruction_with_signers(
            payer,
            instruction,
            &[mint_freeze_authority],
        )
        .await?;
        Ok(())
    }

    pub async fn process_spl_token_transfer(
        &mut self,
        payer: &Keypair,
        owner: &Keypair,
        source_token_account: &Pubkey,
        destination_token_account: &Pubkey,
        amount: u64,
    ) -> Result<()> {
        let instruction = transfer(
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            source_token_account,
            destination_token_account,
            &owner.pubkey(),
            &[],
            amount,
        )?;
        self.process_instruction_with_signers(payer, instruction, &[owner])
            .await?;
        Ok(())
    }

    pub async fn process_spl_token_burn(
        &mut self,
        payer: &Keypair,
        owner: &Keypair,
        source_token_account: &Pubkey,
        mint: &Pubkey,
        amount: u64,
    ) -> Result<()> {
        let instruction = burn(
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            source_token_account,
            mint,
            &owner.pubkey(),
            &[],
            amount,
        )?;
        self.process_instruction_with_signers(payer, instruction, &[owner])
            .await?;
        Ok(())
    }

    pub async fn process_spl_token_account_new(
        &mut self,
        payer: &Keypair,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> Result<Pubkey> {
        let rent_space = Account::LEN;
        let rent_minimum_lamports =
            self.get_sysvar_rent().await?.minimum_balance(rent_space);
        let account = Keypair::new();
        let instruction_create = create_account(
            &payer.pubkey(),
            &account.pubkey(),
            rent_minimum_lamports,
            u64::try_from(rent_space)?,
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
        );
        let instruction_init = initialize_account(
            &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
            &account.pubkey(),
            mint,
            owner,
        )?;
        self.process_instructions_with_signers(
            payer,
            &[instruction_create, instruction_init],
            &[&account],
        )
        .await?;
        Ok(account.pubkey())
    }

    pub async fn get_spl_token_mint(
        &mut self,
        mint: &Pubkey,
    ) -> Result<Option<Mint>> {
        self.get_account_data_unpacked::<Mint>(mint).await
    }

    pub async fn get_spl_token_account(
        &mut self,
        token_account: &Pubkey,
    ) -> Result<Option<Account>> {
        self.get_account_data_unpacked::<Account>(token_account)
            .await
    }

    pub fn convert_spl_token_amount_to_ui_amount(
        token_amount: u64,
        mint_decimals: u8,
    ) -> f64 {
        (token_amount as f64) / 10f64.powi(i32::from(mint_decimals))
    }

    pub fn convert_ui_amount_to_spl_token_amount(
        ui_amount: f64,
        mint_decimals: u8,
    ) -> u64 {
        (ui_amount * 10f64.powi(i32::from(mint_decimals))) as u64
    }
}
