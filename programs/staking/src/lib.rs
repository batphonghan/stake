use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    log::{sol_log_compute_units, sol_log_params, sol_log_slice},
    pubkey::Pubkey,
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod staking {

    use super::*;
    pub fn init(ctx: Context<InitMessage>, authority: Pubkey, amount: u64) -> ProgramResult {
        let ref mut data = ctx.accounts.token;
        data.amount = amount;
        data.authority = authority;

        Ok(())
    }

    pub fn deposit(ctx: Context<DepositMessage>, amount: u64) -> ProgramResult {
        let ref mut data = ctx.accounts.token;
        data.amount += amount;

        Ok(())
    }

    pub fn withdraw(ctx: Context<WithDraw>, amount: u64) -> ProgramResult {
        let ref mut data = ctx.accounts.token;
        data.amount -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMessage<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub token: Account<'info, Token>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositMessage<'info> {
    #[account(mut, has_one = authority)]
    pub token: Account<'info, Token>,

    pub authority: Signer<'info>,
}
#[derive(Accounts)]
pub struct WithDraw<'info> {
    #[account(mut, has_one = authority)]
    pub token: Account<'info, Token>,

    pub authority: Signer<'info>,
}

#[account]
pub struct Token {
    pub authority: Pubkey,
    pub amount: u64,
}
