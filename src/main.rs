use solana_sdk::{
    instruction::{AccountMeta, Instruction},    
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::read_keypair_file,
    transaction::Transaction,
};

use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcSendTransactionConfig,
};

use solana_clap_utiils::key_pair;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SystemProgramTransferInstructionData {
    pub descriminator: u32,
    pub lamports: u64,    
}  


fn main() {
    println!("Hello, rust solana!");

    let key_pair = Keypair:: read_keypair_file("rY2jbCKj45jxLyCWbvJumkCfSixdMh2TNYmPQUS1fzK.json").unwrap();
    let my_pubkey = keypair.try_pubkey().unwrap();
    let random_pubkey =  Pubkey::new_unique();
    println!("My pubkey: {}", my_pubkey);

    let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
    let airdrop_signature = rpc.request_airdrop(&my_pubkey, 1000000000).unwrap();
    println!("Airdrop signature: {}", airdrop_signature);
    let balance = rpc.get_balance(&my_pubkey).unwrap();
    println!("My balance: {}", balance);

    let system_program_id = Pubkey::default();
    let lamports_per_sol = 1000000000;
    let instruction_data_struct = SystemProgramTransferInstructionData{descriminator: 2, lamports: 0.1 * lamports_per_sol};
    let mut instruction_data = Vec::new();
    instruction_data_struct.serialize(&mut instruction_data).unwrap();
    let instruction_account = vec![
        AccountMeta{pubkey: my_pubkey, is_signer: true, is_writable: true},
        AccountMeta{pubkey: random_pubkey, is_signer: false, is_writable: true},
    ];

    let ix = Instruction::new_with_bytes(system_program_id, &instruction_data, instruction_account);

    let signers = &[&keypair];
    let blockhash = rpc.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&my_pubkey),
        signers,
        blockhash,
    );

    let mut config = RpcSendTransactionConfig::default();
    config.skip_preflight= true;
    let sx = rpc.RpcSendTransactionConfig(&tx,config).unwrap();
    println!("signature{}", sx);
 

}
