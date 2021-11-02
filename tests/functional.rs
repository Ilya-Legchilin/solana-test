use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
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
    let pubkey_pda_x = Pubkey::new_unique();
    let pubkey_pda_y = Pubkey::new_unique();
    let pubkey_user_a = Pubkey::new_unique();
    let pubkey_user_b = Pubkey::new_unique();
    let lamp_pda_x = 10_000_000; // these amounts are not actual amount of tokens but 0.00001 part 
    let lamp_pda_y = 100_000_000;
    let lamp_user_a = 1_000_000;
    let lamp_user_b = 2_000_000;
    let dx:u64 = 1_998;

    let mut program_test = ProgramTest::new(
        "test_aloha",
        program_id,
        processor!(process_instruction),
    );
    program_test.add_account(
        pubkey_pda_x,
        Account {
            lamports: lamp_pda_x,
            owner: program_id,
            ..Account::default()
        },
    );
    program_test.add_account(
        pubkey_user_a,
        Account {
            lamports: lamp_user_a,
            owner: program_id,
            ..Account::default()
        },
    );
    program_test.add_account(
        pubkey_pda_y,
        Account {
            lamports: lamp_pda_y,
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

    let account_a = banks_client.get_account(pubkey_user_a).await.unwrap().unwrap();
    let account_b = banks_client.get_account(pubkey_user_b).await.unwrap().unwrap();
    let account_x = banks_client.get_account(pubkey_pda_x).await.unwrap().unwrap();
    let account_y = banks_client.get_account(pubkey_pda_y).await.unwrap().unwrap();
    println!("LAMP A: {}", account_a.lamports);
    println!("LAMP B: {}", account_b.lamports);
    println!("LAMP X: {}", account_x.lamports);
    println!("LAMP Y: {}", account_y.lamports);
    println!("dx: {}", dx);
    
     let v = Data {
        flag: true,
        amount: dx,
    }; 
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &v,
            vec![
                AccountMeta::new(pubkey_user_a, false),
                AccountMeta::new(pubkey_user_b, false),
                AccountMeta::new(pubkey_pda_x, false),
                AccountMeta::new(pubkey_pda_y, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let account_a = banks_client.get_account(pubkey_user_a).await.unwrap().unwrap();
    let account_b = banks_client.get_account(pubkey_user_b).await.unwrap().unwrap();
    let account_x = banks_client.get_account(pubkey_pda_x).await.unwrap().unwrap();
    let account_y = banks_client.get_account(pubkey_pda_y).await.unwrap().unwrap();
    println!("LAMP A: {}", account_a.lamports);
    println!("LAMP B: {}", account_b.lamports);
    println!("LAMP X: {}", account_x.lamports);
    println!("LAMP Y: {}", account_y.lamports);
}
