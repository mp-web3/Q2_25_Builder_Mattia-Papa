// ===== ESCROW TAKE INSTRUCTION IMPLEMENTATION GUIDELINES =====
//
// This file implements the "take" side of the escrow, where the taker accepts
// the trade offered by the maker.
//
// STEP 1: IMPORTS
// - You'll need the same imports as in make.rs, plus any additional ones
// - Make sure to import the Escrow state

// STEP 2: DEFINE THE TAKE ACCOUNTS STRUCT
// Accounts needed:
// - taker: Signer - The account taking the escrow offer (must sign)
// - maker: AccountInfo - Original escrow creator (to send tokens to)
//
// - mint_a: InterfaceAccount<Mint> - The token being offered by maker
// - mint_b: InterfaceAccount<Mint> - The token being requested by maker
//
// - taker_ata_a: InterfaceAccount<TokenAccount> - Where taker receives token A
// - taker_ata_b: InterfaceAccount<TokenAccount> - Source of taker's token B
// - maker_ata_b: InterfaceAccount<TokenAccount> - Where maker receives token B
//
// - escrow: Account<Escrow> - The escrow state account (verify using seeds)
// - vault: InterfaceAccount<TokenAccount> - Holds the deposited tokens
//
// - Programs: token_program, associated_token_program, system_program
//
// Make sure to verify constraints on all accounts:
// - escrow should use seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()]
// - vault should be associated with escrow for token_a
// - taker_ata_a should be the taker's account for mint_a
// - taker_ata_b should be the taker's account for mint_b
// - maker_ata_b should be the maker's account for mint_b

// STEP 3: IMPLEMENT TAKE FUNCTIONALITY
// The take implementation should:
// 1. Transfer token B from taker to maker (using amount from escrow.receive)
// 2. Transfer ALL token A from vault to taker
// 3. Close the vault account and return rent to maker
// 4. Close the escrow account and return rent to maker
//
// Use CPI (Cross-Program Invocation) to:
// - Call transfer_checked for token B from taker to maker
// - Call transfer_checked for token A from vault to taker (using PDA signing)
// - Use close_account to close the vault
//
// For PDA signing, you'll need to use:
// - seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()]
// - bump = escrow.bump

// STEP 4: ADD ENTRY POINT IN lib.rs
// Don't forget to add this instruction to the main program in lib.rs
// pub fn take(ctx: Context<Take>) -> Result<()> {
//   ctx.accounts.execute_trade()
// }
