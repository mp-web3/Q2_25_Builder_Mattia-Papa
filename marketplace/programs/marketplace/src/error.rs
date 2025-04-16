use anchor_lang::prelude::*;

/// Errors that can occur in the marketplace program
#[error_code]
pub enum MarketplaceError {
    /// Fee percentage is invalid
    /// The maximum fee is 100% (10000 basis points)
    #[msg("Fee percentage is invalid. Maximum fee is 100% (10000 basis points).")]
    InvalidFee,

    /// Overflow occurred in a calculation
    #[msg("Arithmetic overflow in calculation")]
    Overflow,

    /// Underflow occurred in a calculation
    #[msg("Arithmetic underflow in calculation")]
    Underflow,

    /// NFT not verified as part of the required collection
    #[msg("NFT is not verified as part of the required collection")]
    InvalidCollection,

    /// The user is not authorized to perform this action
    #[msg("Unauthorized action")]
    Unauthorized,

    /// The listing has been canceled or already purchased
    #[msg("Listing no longer available")]
    ListingUnavailable,
}
