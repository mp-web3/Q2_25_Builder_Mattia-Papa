use anchor_lang::prelude::*;

declare_id!("Atj9w75qkzaYXi6595sABKLaVYaUTHdLcgs1Dw9JULvs");

#[program]
pub mod create_core_asset_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
