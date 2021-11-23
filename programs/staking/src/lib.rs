
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::{
    log::{sol_log_compute_units, sol_log_params, sol_log_slice},
    pubkey::Pubkey,
};
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};
use std::mem::size_of;

#[program]
pub mod staking {
    use super::*;

    pub fn init(ctx: Context<Init>, bump: Bump) -> ProgramResult {
        let ref mut vault = ctx.accounts.vault;
        vault.bump = bump.vault_bump;
        vault.mint_token = ctx.accounts.mint_token.key();

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        let cpi_accounts = Transfer {
            from: ctx.accounts.depositor.to_account_info().clone(),
            to: ctx.accounts.vault_token.to_account_info().clone(),
            authority: ctx.accounts.owner.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct PoolBumps {
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Bump {
    pub vault_bump: u8,
    pub token_bump: u8,
    pub mint_bump: u8,
}

#[derive(Accounts)]
#[instruction(bump: Bump)]
pub struct Init<'info> {
    // For each token we have one vault
    #[account(
        init,
        seeds = [b"vault", mint_token.key().as_ref(), payer.key().as_ref()],
        bump = bump.vault_bump,
        payer = payer,
        space = size_of::<Vault>() + 8,
    )]
    pub vault: ProgramAccount<'info, Vault>,

    #[account(
        init,
        seeds = [b"vault_token", mint_token.key().as_ref(), vault.key().as_ref()],
        bump = bump.token_bump,
        token::mint = mint_token,
        token::authority = vault,
        payer = payer,
    )]
    pub vault_token: CpiAccount<'info, TokenAccount>,

    #[account(
        init, 
        seeds = [b"vault_mint", mint_token.key().as_ref(), vault.key().as_ref()],
        bump = bump.mint_bump,
        mint::authority = vault,
        mint::decimals = mint_token.decimals,
        payer = payer,
    )]
    pub vault_mint: CpiAccount<'info, Mint>,

    pub mint_token: CpiAccount<'info, Mint>,

    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,

    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    #[account(address = spl_token::ID)]
    pub token_program: AccountInfo<'info>,
}

#[account]
pub struct Vault {
    pub bump: u8,
    pub mint_token: Pubkey,  // The token this vault keep
    pub vault_token: Pubkey, // PDA for this vault keep the token
    pub vault_mint: Pubkey,  // LP token mint
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = owner)]
    depositor: CpiAccount<'info, TokenAccount>,

    #[account(mut, "depositor.mint == vault.mint_token")]
    vault: CpiAccount<'info, Vault>,

    #[account(mut)]
    vault_token: CpiAccount<'info, TokenAccount>,

    #[account(signer)]
    owner: AccountInfo<'info>,

    token_program: AccountInfo<'info>,
}
