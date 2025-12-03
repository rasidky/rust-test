use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

fn main() {
    println!("Solana Order Example");
    println!("====================\n");

    // Connect to Solana devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Create a keypair (in production, load from file)
    let payer = Keypair::new();
    println!("Payer Public Key: {}", payer.pubkey());

    // Example: Create an order instruction
    // This is a simplified example - actual order programs vary
    let program_id = Pubkey::from_str("YourProgramIdHere111111111111111111111111111").unwrap();
    
    // Order data structure (example)
    let order_data = create_order_data(
        100.0,  // price
        10.0,   // quantity
        true,   // is_buy
    );

    // Create instruction
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            // Add other required accounts here
        ],
        data: order_data,
    };

    println!("Order created:");
    println!("  Price: 100.0");
    println!("  Quantity: 10.0");
    println!("  Type: Buy");
    println!("\nNote: This is a template. Replace program_id and accounts with actual values.");

    // Uncomment to send transaction (requires funded account)
    /*
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction successful: {}", signature),
        Err(e) => println!("Transaction failed: {}", e),
    }
    */
}

fn create_order_data(price: f64, quantity: f64, is_buy: bool) -> Vec<u8> {
    // Example order data serialization
    // In production, use borsh or bincode for proper serialization
    let mut data = Vec::new();
    
    // Instruction discriminator (example: 0 for place order)
    data.push(0);
    
    // Add price (as u64, scaled by 1e6 for decimals)
    data.extend_from_slice(&((price * 1_000_000.0) as u64).to_le_bytes());
    
    // Add quantity (as u64, scaled by 1e6 for decimals)
    data.extend_from_slice(&((quantity * 1_000_000.0) as u64).to_le_bytes());
    
    // Add order side (1 for buy, 0 for sell)
    data.push(if is_buy { 1 } else { 0 });
    
    data
}
