use anchor_lang::prelude::*;

declare_id!("EXgVjudPHUUSH1SEbMJ9dgrLEmB3Yuu4BHNAcAYTr3Tg");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
