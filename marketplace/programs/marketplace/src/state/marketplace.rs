use anchor_lang::prelude::*;

/// # Marketplace
/// The central state account that stores configuration for the marketplace
/// This account is a Program Derived Address (PDA) created at initialization
/// and holds information about fees, admin access, and naming
#[account]
pub struct Marketplace {
    /// Public key of the marketplace administrator
    /// The admin has special privileges like setting fees
    pub admin: Pubkey,

    /// Fee percentage in basis points (1/100 of 1%)
    /// Example: 250 = 2.5% fee on each sale
    pub fee: u16,

    /// PDA bump seed for the marketplace account
    /// Used for signing operations by the marketplace PDA
    pub bump: u8,

    /// PDA bump seed for the treasury account
    /// The treasury collects fees from all sales
    pub_treasury_bump: u8, // where fees go. Is a PDA

    /// Name of the marketplace
    /// Used in PDA derivation and for identification
    pub name: String,
}

/// Space calculation for the Marketplace account
/// This determines the amount of space to allocate when creating the account
impl Space for Marketplace {
    // 8 bytes for account discriminator
    // 32 bytes for admin Pubkey
    // 2 bytes for fee (u16)
    // 3 bytes for bumps (3 × u8)
    // 4+32 bytes for String name (4 bytes for length + up to 32 bytes for content)
    const INIT_SPACE: usize = 8 + 32 + 2 + 3 * 1 + (4 + 32);
}
