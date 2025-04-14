use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub seed: u64, // 8 bytes
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16,
    pub locked: bool,
    pub config_bump: u8,
    pub lp_bump: u8,
}

impl Space for Config {
    const INIT_SPACE: usize = 8 + 33 + 32 + 32 + 2 + 2 + 1 + 1 + 1 + 8; // = 118
    // 8 + 1 + 1 + 1 + 2 + 32 + 32 +32 + 1 + 8 = 118
}