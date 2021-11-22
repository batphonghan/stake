#![cfg(feature = "test-bpf")]

use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use assert_matches::assert_matches;
use bph_staking;
use solana_program::system_program;
use solana_program::sysvar;
use solana_program::{program_pack::Pack, pubkey::Pubkey, system_instruction};
use solana_sdk::{signature::Keypair, transport::Result};
use {
    solana_program::instruction::{AccountMeta, Instruction},
    solana_program_test::*,
    std::str::FromStr,
};

use solana_sdk::{commitment_config::CommitmentLevel, signature::Signer, transaction::Transaction};

pub async fn process_ins(
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

pub async fn initialize_mint(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    token_mint: &Keypair,
    authority: &Pubkey,
    decimals: u8,
) {
    let rent = banks_client.get_rent().await.unwrap();
    let token_mint_account_rent = rent.minimum_balance(spl_token::state::Mint::LEN);
    let recent_blockhash = banks_client.get_recent_blockhash().await.unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &token_mint.pubkey(),
                token_mint_account_rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &token_mint.pubkey(),
                authority,
                None,
                decimals,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
        &[payer, token_mint],
        recent_blockhash,
    );

    assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
}

pub async fn create_account(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    token_mint: &Pubkey,
    token_keypair: &Keypair,
    authority: &Pubkey,
) {
    let rent = banks_client.get_rent().await.unwrap();
    let token_account_rent = rent.minimum_balance(spl_token::state::Account::LEN);
    let recent_blockhash = banks_client.get_recent_blockhash().await.unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &token_keypair.pubkey(),
                token_account_rent,
                spl_token::state::Account::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(
                &spl_token::id(),
                &token_keypair.pubkey(),
                &token_mint,
                authority,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
        &[payer, token_keypair],
        recent_blockhash,
    );

    assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
}
