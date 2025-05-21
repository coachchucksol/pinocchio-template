use std::fmt::{Debug, Formatter};

use pinocchio_template_sdk::example_program::example_program_id;
use solana_program::{
    clock::Clock, native_token::sol_to_lamports, pubkey::Pubkey, system_instruction::transfer,
};
use solana_program_test::{BanksClientError, ProgramTest, ProgramTestContext};
use solana_sdk::{
    commitment_config::CommitmentLevel,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};
use spl_token_2022::{
    extension::{ExtensionType, StateWithExtensionsOwned},
    instruction::transfer_checked,
};

pub struct TestBuilder {
    pub context: ProgramTestContext,
}

impl Debug for TestBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestBuilder",)
    }
}

impl TestBuilder {
    pub async fn new() -> Self {
        // $ cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo nextest run
        let mut program_test = ProgramTest::new(
            "pinocchio_template_example_program",
            example_program_id(),
            None,
        );

        program_test.prefer_bpf(true);

        let context = program_test.start_with_context().await;

        Self { context }
    }
    

    pub async fn airdrop(&mut self, to: &Pubkey, lamports: u64) -> Result<(), BanksClientError> {
        let blockhash = self.context.banks_client.get_latest_blockhash().await?;
        self.context.banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(&self.context.payer.pubkey(), to, lamports)],
                    Some(&self.context.payer.pubkey()),
                    &[&self.context.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    pub async fn transfer(&mut self, to: &Pubkey, sol: f64) -> Result<(), BanksClientError> {
        let blockhash = self.context.banks_client.get_latest_blockhash().await?;
        self.context
            .banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(
                        &self.context.payer.pubkey(),
                        to,
                        sol_to_lamports(sol),
                    )],
                    Some(&self.context.payer.pubkey()),
                    &[&self.context.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await
    }

    /// Transfers tokens from the source to the destination
    /// source: the source account - ( not the associated token account )
    /// destination: the destination account - ( not the associated token account )
    pub async fn transfer_token(
        &mut self,
        token_program_id: &Pubkey,
        source: &Keypair,
        destination: &Pubkey,
        mint: &Pubkey,
        amount: u64,
    ) -> Result<(), BanksClientError> {
        let blockhash = self.context.banks_client.get_latest_blockhash().await?;

        let mint_account_raw = self
            .context
            .banks_client
            .get_account(*mint)
            .await?
            .ok_or(BanksClientError::ClientError("failed to get mint account"))?;
        let mint_account =
            StateWithExtensionsOwned::<spl_token_2022::state::Mint>::unpack(mint_account_raw.data)
                .unwrap();

        let source_token_account = get_associated_token_address(&source.pubkey(), mint);
        let destination_token_account = get_associated_token_address(destination, mint);

        self.context
            .banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer_checked(
                        token_program_id,
                        &source_token_account,
                        mint,
                        &destination_token_account,
                        &source.pubkey(),
                        &[],
                        amount,
                        mint_account.base.decimals,
                    )
                    .unwrap()],
                    Some(&self.context.payer.pubkey()),
                    &[&source, &self.context.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await
    }

    pub async fn create_token_account(
        &mut self,
        token_program_id: &Pubkey,
        account: &Keypair,
        pool_mint: &Pubkey,
        owner: &Pubkey,
        extensions: &[ExtensionType],
    ) -> Result<(), BanksClientError> {
        let blockhash = self.context.banks_client.get_latest_blockhash().await?;
        let rent = self.context.banks_client.get_rent().await?;
        let space =
            ExtensionType::try_calculate_account_len::<spl_token_2022::state::Account>(extensions)
                .map_err(|_e| {
                    BanksClientError::ClientError("failed to try calculate account length")
                })?;
        let account_rent = rent.minimum_balance(space);

        let mut instructions = vec![solana_program::system_instruction::create_account(
            &self.context.payer.pubkey(),
            &account.pubkey(),
            account_rent,
            space as u64,
            token_program_id,
        )];

        for extension in extensions {
            match extension {
                ExtensionType::ImmutableOwner => instructions.push(
                    spl_token_2022::instruction::initialize_immutable_owner(
                        token_program_id,
                        &account.pubkey(),
                    )
                    .unwrap(),
                ),
                ExtensionType::TransferFeeAmount
                | ExtensionType::MemoTransfer
                | ExtensionType::CpiGuard
                | ExtensionType::NonTransferableAccount => (),
                _ => unimplemented!(),
            };
        }

        instructions.push(
            spl_token_2022::instruction::initialize_account(
                token_program_id,
                &account.pubkey(),
                pool_mint,
                owner,
            )
            .map_err(|_e| BanksClientError::ClientError("failed to initialize account"))?,
        );

        let mut signers = vec![&self.context.payer, account];
        for extension in extensions {
            match extension {
                ExtensionType::MemoTransfer => {
                    signers.push(&self.context.payer);
                    instructions.push(
                spl_token_2022::extension::memo_transfer::instruction::enable_required_transfer_memos(
                    token_program_id,
                    &account.pubkey(),
                    &self.context.payer.pubkey(),
                    &[],
                ).map_err(|_e| BanksClientError::ClientError("failed to enable required transfer memos"))?
                )
                }
                ExtensionType::CpiGuard => {
                    signers.push(&self.context.payer);
                    instructions.push(
                        spl_token_2022::extension::cpi_guard::instruction::enable_cpi_guard(
                            token_program_id,
                            &account.pubkey(),
                            &self.context.payer.pubkey(),
                            &[],
                        )
                        .map_err(|_e| {
                            BanksClientError::ClientError("failed to enable cpi guard")
                        })?,
                    )
                }
                ExtensionType::ImmutableOwner
                | ExtensionType::TransferFeeAmount
                | ExtensionType::NonTransferableAccount => (),
                _ => unimplemented!(),
            }
        }

        let transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&self.context.payer.pubkey()),
            &signers,
            blockhash,
        );

        self.context
            .banks_client
            .process_transaction(transaction)
            .await
    }

    pub async fn get_token_account(
        &mut self,
        token_account: &Pubkey,
    ) -> Result<spl_token_2022::state::Account, BanksClientError> {
        let account = self
            .context
            .banks_client
            .get_account(*token_account)
            .await?
            .ok_or(BanksClientError::ClientError("failed to get token account"))?;

        let account_info =
            StateWithExtensionsOwned::<spl_token_2022::state::Account>::unpack(account.data)
                .map_err(|_e| BanksClientError::ClientError("failed to unpack"))?;

        Ok(account_info.base)
    }

    pub async fn get_token_mint(
        &mut self,
        token_mint: &Pubkey,
    ) -> Result<spl_token_2022::state::Mint, BanksClientError> {
        let account = self
            .context
            .banks_client
            .get_account(*token_mint)
            .await?
            .ok_or(BanksClientError::ClientError("failed to get token account"))?;

        let account_info =
            StateWithExtensionsOwned::<spl_token_2022::state::Mint>::unpack(account.data)
                .map_err(|_e| BanksClientError::ClientError("failed to unpack"))?;

        Ok(account_info.base)
    }

    /// Mints tokens to an ATA owned by the `to` address
    pub async fn mint_spl_to(
        &mut self,
        mint: &Pubkey,
        to: &Pubkey,
        amount: u64,
        token_program: &Pubkey,
    ) -> Result<(), BanksClientError> {
        let blockhash = self.context.banks_client.get_latest_blockhash().await?;

        let mint_to_ix = if token_program.eq(&spl_token::id()) {
            vec![
                create_associated_token_account_idempotent(
                    &self.context.payer.pubkey(),
                    to,
                    mint,
                    token_program,
                ),
                spl_token::instruction::mint_to(
                    token_program,
                    mint,
                    &get_associated_token_address(to, mint),
                    &self.context.payer.pubkey(),
                    &[],
                    amount,
                )
                .map_err(|_e| BanksClientError::ClientError("failed to mint to"))?,
            ]
        } else {
            vec![spl_token_2022::instruction::mint_to(
                token_program,
                mint,
                to,
                &self.context.payer.pubkey(),
                &[],
                amount,
            )
            .map_err(|_e| BanksClientError::ClientError("failed to mint to"))?]
        };
        self.context
            .banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &mint_to_ix,
                    Some(&self.context.payer.pubkey()),
                    &[&self.context.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await
    }

    pub async fn create_ata(
        &mut self,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> Result<(), BanksClientError> {
        let blockhash = self.context.banks_client.get_latest_blockhash().await?;
        self.context
            .banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[create_associated_token_account_idempotent(
                        &self.context.payer.pubkey(),
                        owner,
                        mint,
                        &spl_token::id(),
                    )],
                    Some(&self.context.payer.pubkey()),
                    &[&self.context.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await
    }

    pub async fn warp_slot_incremental(
        &mut self,
        incremental_slots: u64,
    ) -> Result<(), BanksClientError> {
        let clock: Clock = self.context.banks_client.get_sysvar().await?;
        self.context
            .warp_to_slot(clock.slot.checked_add(incremental_slots).unwrap())
            .map_err(|_| BanksClientError::ClientError("failed to warp slot"))?;
        Ok(())
    }

    pub async fn warp_to_slot(&mut self, warp_slot: u64) -> Result<(), BanksClientError> {
        self.context
            .warp_to_slot(warp_slot)
            .map_err(|_| BanksClientError::ClientError("failed to warp slot"))?;
        Ok(())
    }

    pub async fn get_current_slot(&mut self) -> Result<u64, BanksClientError> {
        let clock: Clock = self.context.banks_client.get_sysvar().await?;
        Ok(clock.slot)
    }

}
