#![cfg(feature = "test-bpf")]

use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use bph_staking;
use solana_program::system_program;
use solana_program::sysvar;
use solana_sdk::signature::Keypair;
use spl_associated_token_account;
use {
    solana_program::{instruction::Instruction, pubkey::Pubkey},
    solana_program_test::*,
    std::str::FromStr,
};

mod helper;
use helper::{initialize_mint, mint_to, process_ins};

use solana_sdk::signature::Signer;

const INIT_AMOUNT: u64 = 1_000_000;
async fn init_user_token(
    banks_client: &mut BanksClient,
    user_keypair: &Keypair,
    token_keypair: &Keypair,
    payer_keypair: &Keypair,
) -> Pubkey {
    initialize_mint(
        banks_client,
        &payer_keypair,
        &token_keypair,
        &payer_keypair.pubkey(),
        6,
    )
    .await;

    process_ins(
        banks_client,
        &[
            spl_associated_token_account::create_associated_token_account(
                &payer_keypair.pubkey(),
                &user_keypair.pubkey(),
                &token_keypair.pubkey(),
            ),
        ],
        &payer_keypair,
        &[],
    )
    .await
    .ok()
    .unwrap_or_else(|| panic!("Can not create ATA account"));

    let user_ata = spl_associated_token_account::get_associated_token_address(
        &user_keypair.pubkey(),
        &token_keypair.pubkey(),
    );

    mint_to(
        payer_keypair,
        &token_keypair.pubkey(),
        &user_ata,
        INIT_AMOUNT,
        banks_client,
    )
    .await;

    user_ata
}

async fn init_user_vault(
    banks_client: &mut BanksClient,
    vault_mint: Pubkey,
    user_keypair: &Keypair,
    token_keypair: &Keypair,
    payer_keypair: &Keypair,
) -> Pubkey {
    process_ins(
        banks_client,
        &[
            spl_associated_token_account::create_associated_token_account(
                &payer_keypair.pubkey(),
                &user_keypair.pubkey(),
                &vault_mint,
            ),
        ],
        &payer_keypair,
        &[],
    )
    .await
    .ok()
    .unwrap_or_else(|| panic!("Can not create ATA account"));
    let user_vault_ata = spl_associated_token_account::get_associated_token_address(
        &user_keypair.pubkey(),
        &vault_mint,
    );

    user_vault_ata
}

#[tokio::test]
async fn test_init() {
    let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
    let program_test = ProgramTest::new("bph_staking", program_id, processor!(bph_staking::entry));

    let (mut banks_client, payer_keypair, _) = program_test.start().await;

    // Init user and token
    let user_keypair = Keypair::new();
    let token_keypair = Keypair::new();
    let mint_token = token_keypair.pubkey();
    let (vault, vault_bump) = Pubkey::find_program_address(
        &[
            b"vault",
            mint_token.as_ref(),
            payer_keypair.pubkey().as_ref(),
        ],
        &program_id,
    );
    let (vault_token, token_bump) = Pubkey::find_program_address(
        &[b"vault_token", mint_token.as_ref(), vault.as_ref()],
        &program_id,
    );
    let (vault_mint, mint_bump) = Pubkey::find_program_address(
        &[b"vault_mint", mint_token.as_ref(), vault.as_ref()],
        &program_id,
    );

    let user_ata = init_user_token(
        &mut banks_client,
        &user_keypair,
        &token_keypair,
        &payer_keypair,
    )
    .await;

    process_ins(
        &mut banks_client,
        &[Instruction {
            program_id,
            data: bph_staking::instruction::Init {
                bump: bph_staking::Bump {
                    vault_bump,
                    token_bump,
                    mint_bump,
                },
            }
            .data(),
            accounts: bph_staking::accounts::Init {
                vault,
                vault_token,
                mint_token,
                vault_mint,
                payer: payer_keypair.pubkey(),
                rent: sysvar::rent::ID,
                system_program: system_program::id(),
                token_program: spl_token::id(),
            }
            .to_account_metas(None),
        }],
        &payer_keypair,
        &[&payer_keypair],
    )
    .await
    .ok()
    .unwrap_or_else(|| panic!("Can not create Init "));

    let user_vault = init_user_vault(
        &mut banks_client,
        vault_mint,
        &user_keypair,
        &token_keypair,
        &payer_keypair,
    )
    .await;
    process_ins(
        &mut banks_client,
        &[Instruction {
            program_id,
            data: bph_staking::instruction::Deposit {
                amount: INIT_AMOUNT * 2,
            }
            .data(),
            accounts: bph_staking::accounts::Deposit {
                vault,
                depositor: user_ata,
                owner: user_keypair.pubkey(),
                vault_mint,
                user_vault,
                vault_token,
                token_program: spl_token::id(),
            }
            .to_account_metas(None),
        }],
        &payer_keypair,
        &[&user_keypair],
    )
    .await
    .err()
    .unwrap_or_else(|| panic!("Should not success"));

    process_ins(
        &mut banks_client,
        &[Instruction {
            program_id,
            data: bph_staking::instruction::Deposit { amount: 1_000_000 }.data(),
            accounts: bph_staking::accounts::Deposit {
                vault,
                vault_mint,
                depositor: user_ata,
                owner: user_keypair.pubkey(),
                vault_token,
                user_vault,
                token_program: spl_token::id(),
            }
            .to_account_metas(None),
        }],
        &payer_keypair,
        &[&user_keypair],
    )
    .await
    .ok()
    .unwrap_or_else(|| panic!("Can not deposit "));
}
