#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;
declare_id!("7RwCakEJihTYJDvpQHkBnXZBuTgX9x51Wrg5Ut4DMRxm");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
      ctx.accounts.deposit()
    }

}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Escrow::INIT_SPACE,
  payer = payer
  )]
  pub escrow: Account<'info, Escrow>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseEscrow<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub escrow: Account<'info, Escrow>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub escrow: Account<'info, Escrow>,
}

#[account]
#[derive(InitSpace)]
pub struct Escrow {
  count: u8,
}
