//! Program instruction processor

use std::convert::TryFrom;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program::invoke_signed,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    system_instruction,
};
use solana_program::msg;


use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Data {
    pub program_id: Pubkey,
    pub flag: bool,
    pub amount: u64,
    pub x: u64,
    pub y: u64,
}

pub const SIZE: usize = 42;

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let system_program_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let source_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;
    let allocated_x_info = next_account_info(account_info_iter)?;
    let allocated_y_info = next_account_info(account_info_iter)?;
    let Data {
        program_id,
        flag,
        amount,
        x,
        y,
    } = Data::try_from_slice(instruction_data)?;
    let payer_key = payer_info.key;
    if **allocated_x_info.try_borrow_mut_lamports()? == 0 {
        invoke_signed(
            &system_instruction::create_account(payer_key, allocated_x_info.key, x, SIZE as u64, &program_id),
            &[
                payer_info.clone(),
                allocated_x_info.clone(),
            ],
            &[&[b"You pass butter x", &[255]]],
        )?;
    }
    if **allocated_y_info.try_borrow_mut_lamports()? == 0 {
        invoke_signed(
            &system_instruction::create_account(payer_key, allocated_y_info.key, y, SIZE as u64, &program_id),
            &[
                payer_info.clone(),
                allocated_y_info.clone(),
            ],
            &[&[b"You pass butter y", &[254]]],
        )?;
    }

    let k = x * y;
    let dy = y - k/(x + amount);
    msg!("dx: {}, dy: {}", amount, dy);
    **source_info.try_borrow_mut_lamports()? -= amount;
    **allocated_x_info.try_borrow_mut_lamports()? += amount;
    **allocated_y_info.try_borrow_mut_lamports()? -= dy;
    **destination_info.try_borrow_mut_lamports()? += dy;

    Ok(())
}
