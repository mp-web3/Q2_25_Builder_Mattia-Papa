use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::state::Escrow;

// ===== TAKE INSTRUCTION ACCOUNTS =====
// This file implements the "take" side of the escrow, where the taker accepts
// the trade offered by the maker.

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {
    // taker: Signer - The account taking the escrow offer (must sign)
    #[account(mut)]
    pub taker: Signer<'info>,

    // The original creator of the escrow (not a signer in this transaction)
    // Verify this matches the maker pubkey stored in the escrow state
    /// CHECK: Safe because we validate this account's address matches the maker stored in escrow state
    #[account(
        constraint = maker.key() == escrow.maker
    )]
    pub maker: AccountInfo<'info>,

    // mint_a: InterfaceAccount<Mint> - The token being offered by maker
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    // mint_b: InterfaceAccount<Mint> - The token being requested by maker
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    // taker_ata_a: InterfaceAccount<TokenAccount> - Where taker receives token A
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // taker_ata_b: InterfaceAccount<TokenAccount> - Source of taker's token B
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // maker_ata_b: InterfaceAccount<TokenAccount> - Where maker receives token B
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // escrow: Account<Escrow> - The escrow state account (verify using seeds)
    // escrow should use seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()]
    // Mark for closure to the maker after the trade completes
    #[account(
        mut,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        close = maker
    )]
    pub escrow: Account<'info, Escrow>,

    // vault: InterfaceAccount<TokenAccount> - Holds the deposited tokens
    #[account(
        mut,
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

impl<'info> Take<'info> {
    // Main function that orchestrates the entire trade execution
    pub fn execute_trade(&self) -> Result<()> {
        // 1. Transfer token B from taker to maker
        self.transfer_b_to_maker()?;

        // 2. Transfer token A from vault to taker
        self.transfer_a_to_taker()?;

        // 3. Close the vault token account
        self.close_vault()?;

        Ok(())
    }
    // Helper function to transfer token B from taker to maker
    fn transfer_b_to_maker(&self) -> Result<()> {
        // Create CPI to token program for transferring token B
        let cpi_program = self.token_program.to_account_info();

        // Configure the transfer accounts for token B
        let transfer_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, transfer_accounts);

        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)
    }

    // Helper function to transfer token A from vault to taker
    fn transfer_a_to_taker(&self) -> Result<()> {
        // Create CPI to token program for transferring token A
        let cpi_program = self.token_program.to_account_info();

        // Configure the transfer accounts for token A
        let transfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        // Create signer seeds for escrow PDA to authorize the transfer
        let maker_key = self.maker.key();
        let escrow_seed = self.escrow.seed.to_le_bytes();
        let escrow_bump = self.escrow.bump;
        let seeds = &[
            b"escrow",
            maker_key.as_ref(),
            &escrow_seed[..],
            &[escrow_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // Create context with signer seeds
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, transfer_accounts, signer_seeds);

        // Execute the transfer
        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)
    }

    // Helper function to close the vault token account
    fn close_vault(&self) -> Result<()> {
        // Create CPI to token program for closing the account
        let cpi_program = self.token_program.to_account_info();
        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        // Create signer seeds for escrow PDA to authorize the closing
        let maker_key = self.maker.key();
        let escrow_seed = self.escrow.seed.to_le_bytes();
        let escrow_bump = self.escrow.bump;
        let seeds = &[
            b"escrow",
            maker_key.as_ref(),
            &escrow_seed[..],
            &[escrow_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // Create context with signer seeds
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, close_accounts, signer_seeds);

        // Close the vault account
        close_account(cpi_ctx)
    }
}
