use anchor_lang::prelude::*;

/// # Listing
/// Represents an NFT listing on the marketplace
/// Each listing is a PDA derived from the marketplace and NFT mint addresses
/// Stores information about the seller, asset, and price
#[account]
pub struct Listing {
    /// The public key of the NFT owner who created this listing
    pub maker: Pubkey,

    /// The mint address of the NFT being listed
    /// This uniquely identifies the NFT asset
    pub maker_mint: Pubkey, //NFT that is going to be listed on the marketplace by the Maker

    /// The sale price in lamports (SOL)
    pub price: u64,

    /// PDA bump seed for the listing account
    /// Used for verification and signing operations
    pub bump: u8,
}

/// Space calculation for the Listing account
/// This determines the amount of space to allocate when creating the account
impl Space for Listing {
    // 8 bytes for account discriminator
    // 32 bytes × 2 for Pubkeys (maker and maker_mint)
    // 8 bytes for price (u64)
    // 1 byte for bump (u8)
    const INIT_SPACE: usize = 8 + 32 * 2 + 8 + 1;
}
