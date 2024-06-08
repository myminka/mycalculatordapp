use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalculatordapp {
    use super::*;
    pub fn create(ctx: Context<Create>, init_message: String) => ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok();
    }
}
