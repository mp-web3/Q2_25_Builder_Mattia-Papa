#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

// Program ID - Unique identifier for this program on the Solana blockchain
declare_id!("7RwCakEJihTYJDvpQHkBnXZBuTgX9x51Wrg5Ut4DMRxm");

// ===== PROGRAM MODULE =====
// This is the main entry point for the escrow program.
// It defines all the instructions that can be called from clients.
#[program]
pub mod escrow {
    use super::*;

    // The 'make' instruction creates a new escrow trade
    // - seed: A unique value to derive the escrow PDA
    // - deposit: Amount of token A to deposit into escrow
    // - receive: Amount of token B expected in return
    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        // Initialize the escrow data
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        // Deposit the tokens from maker into the vault
        ctx.accounts.deposit(deposit)
    }

    // TODO: Add the 'take' instruction
    // This instruction will allow a taker to accept the trade and complete the escrow

    // TODO: Add the 'refund' instruction
    // This instruction will allow the maker to reclaim their tokens if no taker accepts
}

// ===== EXAMPLE ACCOUNT STRUCTURES =====
// These are not used in the current implementation but demonstrate
// how account validation works in Anchor

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

// NOTE: This is a placeholder struct and is replaced by the actual
// Escrow struct defined in state.rs
#[account]
#[derive(InitSpace)]
pub struct Escrow {
    count: u8,
}
