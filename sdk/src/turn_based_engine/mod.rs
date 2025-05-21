use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    rent::sysvar as sysvar_rent,
    system_program,
};
use turn_based_engine::{
    accounts::config::Config,
    instructions::{
        initialize_config::InitializeConfigIxData,
        GameEngineInstructions,
    },
};

// ----------------------- PROGRAM ID -----------------------
pub fn turn_based_engine_program_id() -> Pubkey {
    turn_based_engine::id().into()
}

// ----------------------- CONFIG -----------------------
pub fn config_address(base: &Pubkey) -> (Pubkey, u8) {
    let seeds = [Config::SEED, &base.to_bytes()];
    Pubkey::find_program_address(&seeds, &turn_based_engine_program_id())
}

pub fn initialize_config_ix(
    base: &Pubkey,
    admin: &Pubkey,
    server: &Pubkey,
    game_fee_bps: u32,
) -> Instruction {
    let program_id = turn_based_engine_program_id();
    let sysvar_rent = sysvar_rent::id();
    let system_program = system_program::id();

    let (config, config_bump) = config_address(base);

    let accounts = vec![
        AccountMeta::new(config, false),
        AccountMeta::new_readonly(*base, false),
        AccountMeta::new(*admin, true),
        AccountMeta::new_readonly(*server, false),
        AccountMeta::new_readonly(sysvar_rent, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    let mut ix_data_bytes = vec![];
    ix_data_bytes.push(GameEngineInstructions::InitializeConfig as u8);
    let ix_data = InitializeConfigIxData {
        config_bump,
        game_fee_bps,
    };
    ix_data_bytes.extend(unsafe { turn_based_engine::utils::to_bytes(&ix_data) });

    Instruction {
        program_id,
        accounts,
        data: ix_data_bytes.to_vec(),
    }
}

