use crate::state::{listing::Listing, marketplace::Marketplace};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        MasterEditionAccount, Metadata,
        MetadataAccount,
    },
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

/// # Delist Instruction
/// Allows the original NFT lister to remove their NFT from sale on the marketplace
///
/// When a user delists their NFT:
/// 1. The NFT is transferred from the vault back to the owner's wallet
/// 2. The listing account is closed, returning rent to the owner
/// 3. The NFT is no longer available for purchase
///
/// Only the original maker (lister) can delist an NFT, which is enforced
/// through constraint verification on the listing account.
#[derive(Accounts)]
pub struct Delist<'info> {
    /// The original maker (seller) who listed the NFT
    /// Must sign the transaction to authorize delisting
    /// Will receive the NFT back and receive the rent from closing the listing
    #[account(mut)]
    pub maker: Signer<'info>,

    /// The marketplace account this listing belongs to
    /// Used for verifying listing PDA seeds and program consistency
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// The mint address of the NFT being delisted
    /// Used to identify the specific NFT and for PDA derivation
    pub maker_mint: InterfaceAccount<'info, Mint>,

    /// The maker's token account that will receive the NFT back
    /// This is where the NFT will be returned to after delisting
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    /// The vault token account currently holding the NFT
    /// Owned by the listing PDA, which authorizes the return transfer
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// The listing account that will be closed
    /// This account must be owned by the maker (verified through constraint)
    /// Will be closed and rent returned to maker
    #[account(
        mut,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
        constraint = listing.maker == maker.key(),
        close = maker
    )]
    pub listing: Account<'info, Listing>,

    /// The mint address of the collection this NFT belongs to
    /// Included for reference and consistency with other instructions
    pub collection_mint: InterfaceAccount<'info, Mint>,

    /// Metadata account of the NFT being delisted
    /// Included for reference and consistency with other instructions
    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), maker_mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    /// Master edition account of the NFT
    /// Included for reference and consistency with other instructions
    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), maker_mint.key().as_ref(), b"edition"],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,

    /// Metaplex Token Metadata program
    /// Required for metadata account constraints
    pub metadata_program: Program<'info, Metadata>,

    /// Required for associated token account operations
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Required for system operations
    pub system_program: Program<'info, System>,

    /// Required for token operations
    /// Used to transfer NFT from vault back to owner
    pub token_program: Interface<'info, TokenInterface>,
}

/// Implementation of the delist instruction logic
impl<'info> Delist<'info> {
    /// Transfers the NFT from the vault back to the maker's account
    ///
    /// This function:
    /// 1. Sets up a token transfer from vault to maker's account
    /// 2. Creates proper signer seeds for the listing PDA
    /// 3. Executes the transfer with PDA signing
    ///
    /// The vault is owned by the listing PDA, so the transfer must be
    /// authorized using PDA signing derived from marketplace and NFT mint.
    pub fn withdraw_nft(&mut self) -> Result<()> {
        // Get the token program for CPI (Cross-Program Invocation)
        let cpi_program = self.token_program.to_account_info();

        // Set up the transfer accounts
        // from: vault holding the NFT
        // to: maker's token account to receive the NFT
        // authority: listing PDA (controls the vault)
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        // Store keys in variables to prevent temporary value dropped errors
        // These are needed for PDA signing derivation
        let marketplace_key = self.marketplace.key();
        let maker_mint_key = self.maker_mint.key();

        // Create signer seeds for the listing PDA
        // These match the seeds used to create the listing account
        // and allow the program to sign as the listing PDA
        let seeds = &[
            marketplace_key.as_ref(),
            maker_mint_key.as_ref(),
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // Create the CPI context with signer seeds
        // This allows the program to sign the transaction as the listing PDA
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Execute the transfer
        // Amount is 1 for NFTs as they have quantity of 1
        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)
    }

    /// Execute the delisting process
    ///
    /// This is the main entry point for the delist instruction.
    /// It orchestrates the complete delisting process:
    /// 1. Transfers the NFT back to the original owner
    /// 2. Closes the listing account (handled by account constraints)
    ///
    /// The listing account closure is automatic due to the 'close = maker'
    /// constraint in the account validation.
    pub fn execute_delist(&mut self) -> Result<()> {
        // Transfer the NFT back to the maker
        self.withdraw_nft()?;

        // The listing account will be automatically closed due to the close constraint
        // This returns the rent exempt balance to the maker
        Ok(())
    }
}
