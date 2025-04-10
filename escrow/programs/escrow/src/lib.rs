use anchor_lang::prelude::*;

declare_id!("7RwCakEJihTYJDvpQHkBnXZBuTgX9x51Wrg5Ut4DMRxm");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
