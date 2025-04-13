use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{TokenAccount, TokenInterface, Mint, TransferChecked, transfer_checked}};

use crate::state::Escrow;

// ===== MAKE INSTRUCTION ACCOUNTS =====
// This struct defines all the accounts needed for the 'make' instruction
// which creates a new escrow offer from the maker
#[derive(Accounts)]
#[instruction(seed: u64)] // This makes the instruction arguments available for account constraints
pub struct Make <'info> {
    // The maker is the account creating the escrow offer
    // They must sign the transaction and will pay for account creation
    #[account(mut)]
    pub maker: Signer<'info>,

    // Token A mint - this is the token type being offered by the maker
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    // Token B mint - this is the token type the maker wants to receive
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    // The maker's associated token account for token A
    // This is where the tokens being offered will come from
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // The escrow state account (PDA) that stores trade information
    // We create this account using a seed derived from the maker's key and a nonce
    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + Escrow::INIT_SPACE
    )]
    pub escrow: Account<'info, Escrow>,

    // The vault token account that will hold the maker's tokens in escrow
    // This is an associated token account owned by the escrow PDA
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
        
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // Required programs
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// Implementation of the Make instruction functionality
impl<'info> Make<'info> {
    // Initialize the escrow state data with trade parameters
    pub fn init_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> {
        // Store all the escrow details in the escrow account
        self.escrow.set_inner(Escrow {
            seed,                      // Random seed for PDA derivation
            maker: self.maker.key(),   // Maker's public key for ownership verification
            mint_a: self.mint_a.key(), // Token A mint (what maker is offering)
            mint_b: self.mint_b.key(), // Token B mint (what maker wants in return)
            receive,                   // Amount of token B expected in return
            bump: bumps.escrow,        // Bump seed for the escrow PDA
        });
        Ok(())
    }

    // Deposit tokens from maker's account to the escrow vault
    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        // Set up the CPI (Cross-Program Invocation) to the token program
        let cpi_program = self.token_program.to_account_info();
        
        // Configure the transfer accounts for token A
        let transfer_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),    // Source: maker's token A account
            mint: self.mint_a.to_account_info(),         // Token mint for verification
            to: self.vault.to_account_info(),            // Destination: escrow vault
            authority: self.maker.to_account_info()      // Signer: maker authorizes the transfer
        };
        
        // Create the CPI context (no signing needed since maker is a direct signer)
        let cpi_ctx = CpiContext::new(cpi_program, transfer_accounts);
        
        // Execute the token transfer, specifying the amount and decimal places
        transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)
    }
}
