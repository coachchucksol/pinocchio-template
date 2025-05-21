#[cfg(test)]
mod tests {
    use solana_program::{pubkey::Pubkey};
    use solana_sdk::{commitment_config::CommitmentLevel, signature::{Keypair, Signer}, transaction::Transaction};
    use solarcade_xyz_sdk::turn_based_engine::{config_address, initialize_config_ix};

    use crate::fixtures::{fixture::TestBuilder};

    #[tokio::test]
    async fn test_program_ok() {
        let fixture = TestBuilder::new().await;
        let turn_based_engine_program_id: Pubkey = turn_based_engine::id().into();

        let account = fixture.context.banks_client.get_account(turn_based_engine_program_id).await.unwrap();

        assert!(account.is_some());
        assert!(account.unwrap().data.len() > 0);
    }

    #[tokio::test]
    async fn test_init_config_ok() {
        let fixture = TestBuilder::new().await;
        
        let admin = fixture.context.payer.insecure_clone();
        let base = Keypair::new();
        let server = Keypair::new();
        let game_fee_bps = 100;
        
        let (config, config_bump) = config_address(&base.pubkey());

        let ix = initialize_config_ix(&base.pubkey(), &admin.pubkey(), &server.pubkey(), game_fee_bps);

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
        let config_account = unsafe { turn_based_engine::utils::load_account::<turn_based_engine::accounts::config::Config>(&config_account_data_raw).unwrap() };

        assert_eq!(config_account.game_fee_bps(), game_fee_bps);
        assert_eq!(config_account.bump(), config_bump);

    }
}