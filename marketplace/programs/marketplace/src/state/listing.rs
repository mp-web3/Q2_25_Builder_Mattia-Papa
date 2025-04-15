use anchor_lang::prelude::*;

// Who listed what asset
#[account]
pub struct Listing {
    pub maker: Pubkey, 
    pub maker_mint: Pubkey, //NFT that is going to be listed on the marketplace by the Maker
    pub price: u64,
    pub bump: u8,
}

impl Space for Listing {
    const INIT_SPACE: usize = 8 + 32*2 + 8 + 1;
}