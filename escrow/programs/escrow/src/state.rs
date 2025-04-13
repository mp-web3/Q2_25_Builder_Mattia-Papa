use anchor_lang::prelude::*;

// ===== ESCROW STATE ACCOUNT =====
// This struct defines the data stored in the escrow account
// It contains all the information needed to execute the trade
#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,      // Random seed used for PDA derivation
    pub maker: Pubkey,  // The public key of the escrow creator
    pub mint_a: Pubkey, // Token A mint (what maker is offering)
    pub mint_b: Pubkey, // Token B mint (what maker wants in return)
    pub receive: u64,   // Amount of Token B expected from the taker
    pub bump: u8,       // Bump seed for PDA - needed for singing during take
}
