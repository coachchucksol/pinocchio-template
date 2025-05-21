#[cfg(test)]
mod tests {
    use solana_program::{pubkey::Pubkey};
    use solana_sdk::{commitment_config::CommitmentLevel, instruction::{AccountMeta, Instruction}, rent::sysvar as sysvar_rent, signature::{Keypair, Signer}, system_program, transaction::Transaction};

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
        
        let turn_based_engine_program_id: Pubkey = turn_based_engine::id().into();
        let admin = fixture.context.payer.insecure_clone();
        let base = Keypair::new();
        let server = Keypair::new();
        let sysvar_rent = sysvar_rent::id();
        let system_program = system_program::id();
        
        let seeds = [turn_based_engine::accounts::config::Config::SEED, &base.pubkey().to_bytes()];
        // let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (config, config_bump) = Pubkey::find_program_address(&seeds, &turn_based_engine_program_id);


        // config, base, admin, server, sysvar_rent, system_program
        let accounts = vec![
            AccountMeta::new(config, false),
            AccountMeta::new_readonly(base.pubkey(), false),
            AccountMeta::new(admin.pubkey(), true),
            AccountMeta::new_readonly(server.pubkey(), false),
            AccountMeta::new_readonly(sysvar_rent, false), //TODO take out
            AccountMeta::new_readonly(system_program, false),
        ];

        let mut ix_data_bytes = vec![];
        ix_data_bytes.push(turn_based_engine::instructions::GameEngineInstructions::InitializeConfig as u8);
        let ix_data = turn_based_engine::instructions::initialize_config::InitializeConfigIxData {
            config_bump,
            game_fee_bps: 100,
        };
        ix_data_bytes.extend(unsafe { turn_based_engine::utils::to_bytes(&ix_data) });

        let ix = Instruction {
            program_id: turn_based_engine_program_id,
            accounts,
            data: ix_data_bytes.to_vec(),
        };

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

        assert_eq!(config_account.game_fee_bps(), 100);
        assert_eq!(config_account.bump(), config_bump);


    }
}