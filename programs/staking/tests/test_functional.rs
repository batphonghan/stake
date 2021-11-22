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
use helper::{initialize_mint, process_ins};

use solana_sdk::signature::Signer;

#[tokio::test]
async fn test_init() {
    let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
    let program_test = ProgramTest::new("bph_staking", program_id, processor!(bph_staking::entry));

    let (mut banks_client, payer, _) = program_test.start().await;

    // Init user and token
    let user_authority = Keypair::new();
    let token_keypair = Keypair::new();
    initialize_mint(
        &mut banks_client,
        &payer,
        &token_keypair,
        &user_authority.pubkey(),
        6,
    )
    .await;

    // helper::create_account(
    //     &mut banks_client,
    //     &payer,
    //     &token_keypair.pubkey(),
    //     &token_keypair,
    //     &user_authority.pubkey(),
    // )
    // .await;

    return;

    process_ins(
        &mut banks_client,
        &[
            spl_associated_token_account::create_associated_token_account(
                &payer.pubkey(),
                &user_authority.pubkey(),
                &token_keypair.pubkey(),
            ),
        ],
        &payer,
        &[],
    )
    .await
    .ok()
    .unwrap_or_else(|| panic!("Can not create ATA account"));
    return;

    let (vault, vault_bump) =
        Pubkey::find_program_address(&[b"vault", payer.pubkey().as_ref()], &program_id);

    let mint_token = token_keypair.pubkey();
    initialize_mint(
        &mut banks_client,
        &payer,
        &token_keypair,
        &payer.pubkey(),
        6,
    )
    .await;

    let (vault_token, token_bump) = Pubkey::find_program_address(
        &[b"vault_token", mint_token.as_ref(), vault.as_ref()],
        &program_id,
    );

    let (vault_mint, mint_bump) = Pubkey::find_program_address(
        &[b"vault_mint", mint_token.as_ref(), vault.as_ref()],
        &program_id,
    );

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
                payer: payer.pubkey(),
                rent: sysvar::rent::ID,
                system_program: system_program::id(),
                token_program: spl_token::id(),
            }
            .to_account_metas(None),
        }],
        &payer,
        &[&payer],
    )
    .await
    .ok()
    .unwrap_or_else(|| panic!("Can not create Init "));

    // let assert_eq!(
    //     process_ins(
    //         &mut banks_client,
    //         &[Instruction {
    //             program_id,
    //             data: bph_staking::instruction::Deposit { amount: 1 }.data(),
    //             accounts: bph_staking::accounts::Deposit {
    //                 vault: user_vault,
    //                 depositor: user_vault,
    //                 owner: user_vault,
    //                 token_program: user_vault,
    //             }
    //             .to_account_metas(None),
    //         }],
    //         &payer,
    //         &[&payer],
    //     )
    //     .await
    //     .is_ok(),
    //     true,
    // );
}
