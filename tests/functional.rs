use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    solana_program_test::*,
    solana_sdk::{account::Account, signature::Signer, transaction::Transaction},
    test_aloha::processor::process_instruction,
    std::str::FromStr,
};

use test_aloha::processor::Data;

#[tokio::test]
async fn test_lamport_transfer() {
    let program_id = Pubkey::from_str("TransferLamports111111111111111111111111111").unwrap();
    let (allocated_pubkey_x, bump_seed_x) =
        Pubkey::find_program_address(&[b"You pass butter x"], &program_id);
    let (allocated_pubkey_y, bump_seed_y) =
        Pubkey::find_program_address(&[b"You pass butter y"], &program_id);
    let pubkey_user_a = Pubkey::new_unique();
    let pubkey_user_b = Pubkey::new_unique();
    let lamp_user_a = 1_000_000;
    let lamp_user_b = 2_000_000;
    let lamp_x = 10_000_000;
    let lamp_y = 20_000_000;
    let dx:u64 = 1_998;

    let mut program_test = ProgramTest::new(
        "test_aloha",
        program_id,
        processor!(process_instruction),
    );    println!("bump seed x: {}", bump_seed_x);
    program_test.add_account(
        pubkey_user_a,
        Account {
            lamports: lamp_user_a,
            owner: program_id,
            ..Account::default()
        },
    );
    program_test.add_account(
        pubkey_user_b,
        Account {
            lamports: lamp_user_b,
            owner: program_id,
            ..Account::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let pubkey_payer = payer.pubkey();
    
     let v = Data {
        program_id: program_id,
        flag: true,
        amount: dx,
        x: lamp_x,
        y: lamp_y,
    };
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &v,
            vec![
                AccountMeta::new(system_program::id(), false),
                AccountMeta::new(pubkey_payer, false),
                AccountMeta::new(pubkey_user_a, false),
                AccountMeta::new(pubkey_user_b, false),
                AccountMeta::new(allocated_pubkey_x, false),
                AccountMeta::new(allocated_pubkey_y, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
   
    let account_user_a = banks_client.get_account(pubkey_user_a).await.unwrap().unwrap();
    let account_user_b = banks_client.get_account(pubkey_user_b).await.unwrap().unwrap();
    let account_allocated_x = banks_client.get_account(allocated_pubkey_x).await.unwrap().unwrap();
    let account_allocated_y = banks_client.get_account(allocated_pubkey_y).await.unwrap().unwrap();
    let lamp_user_a = account_user_a.lamports;
    let lamp_user_b = account_user_b.lamports;
    let lamp_all_x = account_allocated_x.lamports;
    let lamp_all_y = account_allocated_y.lamports;
    println!("lamp user a: {}", lamp_user_a);
    println!("lamp user b: {}", lamp_user_b);
    println!("lamp allocated x: {}", lamp_all_x);
    println!("lamp allocated y: {}", lamp_all_y);
}
