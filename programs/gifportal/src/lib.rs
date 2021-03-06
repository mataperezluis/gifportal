use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod gifportal {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult{
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0; 
        Ok(())
    }
    pub fn add_gif(ctx: Context<AddGif>) -> ProgramResult{
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs += 1; 
        Ok(())

    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info>{
    #[account(init, payer=user, space=9000)]
    pub base_account: Account<'info,BaseAccount>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddGif<'info>{
        #[account(mut)]
        pub base_account: Account<'info, BaseAccount>
}

#[account]
pub struct BaseAccount{
    pub total_gifs: i64, 
}