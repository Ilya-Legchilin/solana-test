//! Program instruction processor

use std::convert::TryFrom;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use solana_program::msg;


use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Data {
    pub flag: bool,
    pub amount: u64,
}

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let source_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;
    let pda_x_info = next_account_info(account_info_iter)?;
    let pda_y_info = next_account_info(account_info_iter)?;
    let Data {flag, amount} = Data::try_from_slice(instruction_data)?;
    println!("data: {:?}", instruction_data);
    let x = **pda_x_info.try_borrow_mut_lamports()?;
    let y = **pda_y_info.try_borrow_mut_lamports()?;
    let k = x * y;
    let dy = y - k/(x + amount);
    msg!("dx: {}, dy: {}", amount, dy);
    **source_info.try_borrow_mut_lamports()? -= amount;
    **pda_x_info.try_borrow_mut_lamports()? += amount;
    **destination_info.try_borrow_mut_lamports()? += dy;
    **pda_y_info.try_borrow_mut_lamports()? -= dy;

    Ok(())
}
