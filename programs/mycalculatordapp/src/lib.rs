use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalculatordapp {
    use super::*;
    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        return Ok(());
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Calculator{
    pub greeting: String,
    pub result: i64,
    pub remainder: i64
}