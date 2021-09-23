use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod staking {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        msg!("HIIIIIIIIIIIIIIII");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
