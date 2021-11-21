#![cfg(feature = "test-bpf")]

use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use assert_matches::assert_matches;
use bph_staking;
use solana_program::system_program;
use solana_sdk::{signature::Keypair, system_instruction, transport::Result};
use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_program_test::*,
    std::str::FromStr,
};

use solana_sdk::{commitment_config::CommitmentLevel, signature::Signer, transaction::Transaction};
#[tokio::test]
async fn test_init() {
    let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
    let program_test = ProgramTest::new("bph_staking", program_id, processor!(bph_staking::entry));

    let (mut banks_client, payer, _) = program_test.start().await;

    let counter_keypair = Keypair::new();

    let rs = process_ins(
        &mut banks_client,
        &[Instruction {
            program_id,
            data: bph_staking::instruction::Init { amount: 1 }.data(),
            accounts: bph_staking::accounts::Init {
                counter: counter_keypair.pubkey(),
                admin: payer.pubkey(),
                system_program: system_program::id(),
            }
            .to_account_metas(None),
        }],
        &payer,
        &[&counter_keypair],
    )
    .await;

    assert_matches!(rs, Ok(()));
}

async fn process_ins(
    banks_client: &mut BanksClient,
    instructions: &[Instruction],
    payer: &Keypair,
    signers: &[&Keypair],
) -> Result<()> {
    let recent_blockhash = banks_client.get_recent_blockhash().await.unwrap();

    let mut all_signers = vec![payer];
    all_signers.extend_from_slice(signers);

    let mut tx = Transaction::new_with_payer(instructions, Some(&payer.pubkey()));
    if let Err(e) = tx.try_sign(&all_signers, recent_blockhash) {
        panic!(">>> Transaction::sign failed with error {:?}", e)
    }

    banks_client
        .process_transaction_with_commitment(tx, CommitmentLevel::Finalized)
        .await
}
