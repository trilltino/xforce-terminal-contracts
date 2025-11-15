//! Example: Integrating Batch Swap Router with Terminal
//! 
//! This example shows how to use the batch swap router client in your terminal application

use xforce_terminal_contracts_client::{
    create_client,
    BatchSwapRouterClient,
    get_batch_swap_router_program_id,
    SwapParams,
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize wallet (in your terminal, load from config or user input)
    let wallet = Keypair::new(); // Replace with actual wallet loading
    
    // Connect to Solana cluster
    let cluster_url = "http://127.0.0.1:8899"; // Localnet
    // For devnet: "https://api.devnet.solana.com"
    // For mainnet: "https://api.mainnet-beta.solana.com"
    
    // Create client
    let client = create_client(cluster_url, wallet)
        .map_err(|e| format!("Failed to create client: {}", e))?;
    
    // Get the batch swap router program
    let program_id = get_batch_swap_router_program_id();
    let program = client.program(program_id)
        .map_err(|e| format!("Failed to get program: {}", e))?;
    
    // Create batch swap router client
    let swap_client = BatchSwapRouterClient::new(program);
    
    // Example: Execute batch swap
    let swaps = vec![
        SwapParams {
            input_mint: Pubkey::new_unique(), // Replace with actual mint
            output_mint: Pubkey::new_unique(), // Replace with actual mint
            amount: 1000,
            min_output_amount: 900, // 10% slippage tolerance
        },
        SwapParams {
            input_mint: Pubkey::new_unique(),
            output_mint: Pubkey::new_unique(),
            amount: 2000,
            min_output_amount: 1800,
        },
    ];
    
    println!("Executing batch swap with {} swaps...", swaps.len());
    
    // Note: After building the Anchor program, update this to use the generated IDL types
    match swap_client.batch_swap(swaps) {
        Ok(signature) => {
            println!("Batch swap executed successfully!");
            println!("Transaction signature: {}", signature);
        }
        Err(e) => {
            eprintln!("Failed to execute batch swap: {}", e);
            eprintln!("Note: Build the program first with 'anchor build' to generate IDL types");
            return Err(Box::new(e));
        }
    }
    
    Ok(())
}

