use anchor_lang::prelude::*;

/// Program modules
pub mod constants; // Program constants and configuration values
pub mod error; // Error definitions for the program
pub mod instructions; // Instruction implementations (initialize, list, purchase)
pub mod state; // Program state definitions (marketplace, listing)

pub use error::MarketplaceError;
/// Re-export modules for easier access
pub use instructions::*;
pub use state::*;
declare_id!("4wpttMvYD4JtQrafuxd6ZgYAdaWtvEzM769bq6ptiNB5");

/// # NFT Marketplace Program
///
/// This program allows users to create NFT marketplaces where NFTs can be listed and sold.
/// The program supports:
/// - Creating configurable marketplaces with custom fee structures
/// - Listing NFTs for sale with collection verification
/// - Purchasing NFTs with automatic fee distributions
/// - (Future) Reward token distribution for marketplace participants
#[program]
pub mod marketplace {
    use super::*;

    /// Creates a new marketplace with the specified configuration
    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        // Handle validation
        require!(fee <= 10000, MarketplaceError::InvalidFee); // Max fee is 100% (10000 basis points)

        // Call the implementation function
        ctx.accounts.init(name, fee, &ctx.bumps)
    }
}
