use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    rent::sysvar as sysvar_rent,
    system_program,
};
use pinocchio_template_example_program::{
    accounts::config::Config,
    instructions::{
        initialize_config::InitializeConfigIxData, update_config::UpdateConfigIxData,
    },
};

// ----------------------- PROGRAM ID -----------------------
pub fn example_program_id() -> Pubkey {
    pinocchio_template_example_program::id().into()
}

// ----------------------- CONFIG -----------------------
pub fn config_address(base: &Pubkey) -> (Pubkey, u8) {
    let seeds = [Config::SEED, &base.to_bytes()];
    Pubkey::find_program_address(&seeds, &example_program_id())
}

pub fn initialize_config_ix(
    base: &Pubkey,
    admin: &Pubkey,
    fees_bps: u64,
) -> Instruction {
    let program_id = example_program_id();
    let sysvar_rent = sysvar_rent::id();
    let system_program = system_program::id();

    let (config, config_bump) = config_address(base);

    let accounts = vec![
        AccountMeta::new(config, false),
        AccountMeta::new_readonly(*base, false),
        AccountMeta::new(*admin, true),
        AccountMeta::new_readonly(sysvar_rent, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    let ix_data = InitializeConfigIxData::new(config_bump, fees_bps);
    let ix_data_bytes = unsafe {
        ix_data.to_bytes()
    };

    Instruction {
        program_id,
        accounts,
        data: ix_data_bytes.to_vec(),
    }
}

pub fn update_config_ix(
    base: &Pubkey,
    admin: &Pubkey,
    new_admin: Option<Pubkey>,
    new_fees_bps: Option<u64>,
) -> Instruction {
    let program_id = example_program_id();

    let (config, _) = config_address(base);

    let accounts = vec![
        AccountMeta::new(config, false),
        AccountMeta::new(*admin, true),
    ];

    let ix_data = UpdateConfigIxData::new(
        new_admin.map(|p| p.to_bytes()),
        new_fees_bps,
    );
    let ix_data_bytes = unsafe {
        ix_data.to_bytes()
    };

    Instruction {
        program_id,
        accounts,
        data: ix_data_bytes.to_vec(),
    }
}
