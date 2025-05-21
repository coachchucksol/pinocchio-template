#[cfg(test)]
mod tests {
    use pinocchio_template_sdk::example_program::{ config_address, example_program_id, initialize_config_ix, update_config_ix,};
    use solana_program::{pubkey::Pubkey};
    use solana_sdk::{commitment_config::CommitmentLevel, signature::{Keypair, Signer}, transaction::Transaction};

    use crate::fixtures::{fixture::TestBuilder};

    pub async fn init_config(fixture: &TestBuilder, base: &Keypair, admin: &Keypair, fees_bps: u64) -> (Pubkey, u8) {
        let (config, config_bump) = config_address(&base.pubkey());

        let ix = initialize_config_ix(&base.pubkey(), &admin.pubkey(), fees_bps);

        let blockhash = fixture.context.banks_client.get_latest_blockhash().await.unwrap();
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&fixture.context.payer.pubkey()),
            &[&fixture.context.payer],
            blockhash,
        );

        fixture.context.banks_client.process_transaction_with_preflight_and_commitment(tx, CommitmentLevel::Processed).await.unwrap();

        (config, config_bump)
    }

    #[tokio::test]
    async fn test_program_ok() {
        let fixture = TestBuilder::new().await;
        let example_program_id: Pubkey = example_program_id();

        let account = fixture.context.banks_client.get_account(example_program_id).await.unwrap();

        assert!(account.is_some());
        assert!(account.unwrap().data.len() > 0);
    }

    #[tokio::test]
    async fn test_init_config_ok() {
        let fixture = TestBuilder::new().await;
        
        let admin = fixture.context.payer.insecure_clone();
        let base = Keypair::new();
        let fees_bps = 100;
        
        let (config, config_bump) = init_config(&fixture, &base, &admin, fees_bps).await;

        let config_account_raw = fixture.context.banks_client.get_account(config).await.unwrap().unwrap();
        let config_account_data_raw = config_account_raw.data;
        let config_account = unsafe { pinocchio_template_example_program::utils::load_account::<pinocchio_template_example_program::accounts::config::Config>(&config_account_data_raw).unwrap() };

        assert_eq!(config_account.fees_bps(), fees_bps);
        assert_eq!(config_account.bump(), config_bump);
        assert_eq!(*config_account.base(), base.pubkey().to_bytes());
        assert_eq!(*config_account.admin(), admin.pubkey().to_bytes());
    }

    #[tokio::test]
    async fn test_update_config_ok() {
        let fixture = TestBuilder::new().await;
        
        let admin = fixture.context.payer.insecure_clone();
        let base = Keypair::new();
        let game_fee_bps = 100;

        let (config, config_bump) = init_config(&fixture, &base, &admin, game_fee_bps).await;

        let new_admin = Keypair::new();
        let new_fees_bps = 200;

        let ix = update_config_ix(&base.pubkey(), &admin.pubkey(), Some(new_admin.pubkey()), Some(new_fees_bps));

        let blockhash = fixture.context.banks_client.get_latest_blockhash().await.unwrap();
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&fixture.context.payer.pubkey()),
            &[&fixture.context.payer],
            blockhash,
        );

        fixture.context.banks_client.process_transaction_with_preflight_and_commitment(tx, CommitmentLevel::Processed).await.unwrap();

        let config_account_raw = fixture.context.banks_client.get_account(config).await.unwrap().unwrap();
        let config_account_data_raw = config_account_raw.data;
        let config_account = unsafe { pinocchio_template_example_program::utils::load_account::<pinocchio_template_example_program::accounts::config::Config>(&config_account_data_raw).unwrap() };


        assert_eq!(config_account.fees_bps(), new_fees_bps);
        assert_eq!(config_account.bump(), config_bump);
        assert_eq!(*config_account.base(), base.pubkey().to_bytes());
        assert_eq!(*config_account.admin(), new_admin.pubkey().to_bytes());
    }
}