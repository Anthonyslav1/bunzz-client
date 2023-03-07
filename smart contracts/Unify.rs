// Define the necessary accounts for the contract
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Declare the program ID
solana_program::declare_id!("UNIFY1111111111111111111111111111111111111");

// Define the token struct
struct Token {
    pub total_supply: u64,
    pub decimals: u8,
    pub symbol: [u8; 5],
}

// Create the main function for the program
#[entrypoint]
fn initialize_token_program(
    ctx: &mut Context,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // Check if the token has already been initialized
    let mut token_account_info = next_account_info(accounts)?;
    if token_account_info.owner != &solana_program::system_program::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Create the token struct with initial values
    let mut token = Token {
        total_supply: 1_000_000_000 * 10u64.pow(18),
        decimals: 18,
        symbol: [b'U', b'N', b'I', b'F', b'Y'],
    };

    // Write the token struct to the token account
    let mut data = vec![0; std::mem::size_of::<Token>()];
    token.to_bytes(&mut data[..]);
    let mut token_account_data = token_account_info.try_borrow_mut_data()?;
    token_account_data[..data.len()].copy_from_slice(&data);

    Ok(())
}
