use std::env;
use std::str::FromStr;

// Import the program modules
// use turn_based_engine::accounts;
// use turn_based_engine::instructions;

fn main() {

    // let config = Config::load_config();
    // let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     eprintln!("Usage: {} <command> <keypair_path>", args[0]);
    //     std::process::exit(1);
    // }

    // let command = &args[1];
    // let rpc_url = "http://localhost:8899";
    // let client = RpcClient::new(rpc_url.to_string());
    // let payer = read_keypair_file(&args[2]).unwrap();

    // let program_id = Pubkey::from_str("H7NQGd5ZDZtHJNmCpgyi6b3kuoJpS8mvQCrVrg9yRt9V").unwrap();
    // let (pda_pubkey, bump) = Pubkey::find_program_address(&[b"counter".as_ref()], &program_id);
    
    // // Use instructions from the program module instead of creating manually
    // let instruction = if command == "init" {
    //     instructions::initialize(
    //         program_id,
    //         payer.pubkey(),
    //         pda_pubkey,
    //     )
    // } else if command == "increment" {
    //     instructions::increment(
    //         program_id,
    //         payer.pubkey(),
    //         pda_pubkey,
    //     )
    // } else {
    //     eprintln!("Unknown command: {}", command);
    //     std::process::exit(1);
    // };

    // let recent_blockhash = client.get_latest_blockhash().unwrap();
    // let message = Message::new(&[instruction], Some(&payer.pubkey()));
    // let tx = Transaction::new(&[&payer], message, recent_blockhash);

    // let signature = client.send_and_confirm_transaction(&tx).unwrap();
    // println!("Transaction sent: {signature}");
}
