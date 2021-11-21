use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::{
    log::{sol_log_compute_units, sol_log_params, sol_log_slice},
    pubkey::Pubkey,
};
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod staking {

    use super::*;

    pub fn init(ctx: Context<Init>, bump: u8, amount: u64) -> ProgramResult {
        let ref mut vault = ctx.accounts.vault;
        vault.amount = amount;
        vault.bump = bump;
        vault.admin = *ctx.accounts.admin.key;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        let cpi_accounts = Transfer {
            from: ctx.accounts.depositor.to_account_info().clone(),
            to: ctx.accounts.vault.to_account_info().clone(),
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

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Init<'info> {
    #[account(
        init, 
        seeds = [b"vault", admin.key().as_ref()],
        bump = bump,
        payer = payer,
        space = 8 + 64,
    )]
    pub vault: ProgramAccount<'info, Vault>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub mint_token: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub amount: u64,
    pub bump: u8,
    pub admin: Pubkey,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = owner)]
    depositor: Account<'info, TokenAccount>,

    #[account(mut, "depositor.mint == vault.mint")]
    vault: Account<'info, TokenAccount>,
    #[account(signer)]
    owner: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
}
