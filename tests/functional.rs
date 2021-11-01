use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_program_test::*,
    solana_sdk::{account::Account, signature::Signer, transaction::Transaction},
    spl_example_transfer_lamports::processor::process_instruction,
    std::str::FromStr,
};

#[tokio::test]
async fn test_lamport_transfer() {
    let program_id = Pubkey::from_str("TransferLamports111111111111111111111111111").unwrap();
    let pubkey_pda_x = Pubkey::new_unique();
    let pubkey_pda_y = Pubkey::new_unique();
    let pubkey_user_a = Pubkey::new_unique();
    let pubkey_user_b = Pubkey::new_unique();
    let x = 10_000_000; // these amounts are not actual amount of tokens but 0.00001 part 
    let y = 100_000_000;
    let lamp_user_a = 1_000_000;
    let lamp_user_b = 2_000_000;
    let dx:u64 = 1_998;
    let k = x * y;
    let mut program_test = ProgramTest::new(
        "spl_example_transfer_lamports",
        program_id,
        processor!(process_instruction),
    );
    program_test.add_account(
        pubkey_pda_x,
        Account {
            lamports: x,
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
            lamports: y,
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

    let mut v = [0, 0, 0, 0, 0, 0, 0, 0];
    for i in 0..8 {
        v[i] = dx & (0xFF << i*8);
    }
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &v,
            vec![
                AccountMeta::new(pubkey_user_a, false),
                AccountMeta::new(pubkey_pda_x, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let dy = y - k/(x + dx);
    for i in 0..8 {
        v[i] = dy & (0xFF << i*8);
    }
    transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &v,
            vec![
                AccountMeta::new(pubkey_pda_y, false),
                AccountMeta::new(pubkey_user_b, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
    let account_user_a = banks_client.get_account(pubkey_user_a).await.unwrap().unwrap();
    let account_pda_x = banks_client.get_account(pubkey_pda_x).await.unwrap().unwrap();
    let lampx = account_user_a.lamports;
    let lampy = account_pda_x.lamports;
    println!("==============================**** DEBUG INFO ****==============================");
    println!("dx: {} dy: {}", dx, dy);
    println!("Want to withdraw {} pieces of A token", dx);
    println!("Get {} pieces of B token back", dy);
    println!("Via equation: {} = {} - {}/({} + {})", dy, y, k, x, dx);
    println!("LAMP USER_A: {} LAMP PDA_X: {}", lampx, lampy);
    let account_user_b = banks_client.get_account(pubkey_user_b).await.unwrap().unwrap();
    let account_pda_y = banks_client.get_account(pubkey_pda_y).await.unwrap().unwrap();
    let lampx = account_user_b.lamports;
    let lampy = account_pda_y.lamports;
    println!("LAMP USER_B: {} LAMP PDA_Y: {}", lampx, lampy);
    println!("==============================**** DEBUG INFO ****==============================");
}
