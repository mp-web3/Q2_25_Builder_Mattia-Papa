#[allow(unexpected_cfgs)]
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

// Program ID generated during 'anchor init'
// This is the unique identifier for this program on the Solana blockchain
declare_id!("2FmfXbj5gPvLD3vjmKKHMsqo14K3VvzxWcJ1zD5pZ8G9");

// ------------------------------------------------------------------------------------------
// PROGRAM ENTRYPOINTS
// ------------------------------------------------------------------------------------------
// The main program module that contains all the instructions that can be called by clients
// Each instruction processes a specific action and operates on the provided accounts
#[program]
pub mod vault {
    use super::*;

    // Initializes a new vault for a user
    // This creates two Program Derived Addresses (PDAs):
    // 1. vault_state - Stores metadata about the vault (bump seeds)
    // 2. vault - The actual address that will hold the user's funds
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    // Deposits funds from the user's wallet into their vault
    // The amount parameter specifies how many lamports (1 SOL = 1,000,000,000 lamports)
    // to transfer from the user to their vault PDA
    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    // Withdraws funds from the user's vault back to their wallet
    // The amount parameter specifies how many lamports to withdraw
    // This requires a PDA signature since the vault PDA owns the funds
    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    // Closes the vault completely, transferring any remaining funds to the user
    // This also closes the vault_state account, returning its rent-exempt balance to the user
    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        ctx.accounts.close()
    }
}

// ------------------------------------------------------------------------------------------
// ACCOUNT STRUCTURES
// ------------------------------------------------------------------------------------------

// Initialize accounts - Required for the initialize instruction
// Defines the accounts involved in vault initialization
#[derive(Accounts)]
pub struct Initialize<'info> {
    // The user executing the instruction - must sign the transaction
    // 'mut' because the user will pay for the vault_state account creation
    #[account(mut)]
    pub user: Signer<'info>,

    // A PDA that will store metadata about the vault
    // This account is:
    // - Initialized ('init') and paid for by the user
    // - Derived using seeds 'state' and the user's public key
    // - Given space according to VaultState::INIT_SPACE (10 bytes)
    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,

    // The vault PDA itself - this is where the user's funds will be stored
    // This doesn't need 'init' because it's a native system account (like a wallet)
    // It's derived using seeds 'vault' and the user's public key
    #[account(
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>, // Native SOL holding account (like a wallet)

    // Required for creating accounts and transferring SOL
    pub system_program: Program<'info, System>,
}

// Implementation for the Initialize struct
// Contains the actual functionality for initializing a vault
impl<'info> Initialize<'info> {
    // Stores the bump seeds for both PDAs in the vault_state account
    // These are needed later for withdrawals and closing to reconstruct the PDA signatures
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}

// Struct defining the data stored in the vault_state account
// This is the on-chain data that gets saved when vault_state is created
#[account]
pub struct VaultState {
    // Bump seed for the vault PDA - needed to recreate the signing PDA
    pub vault_bump: u8, // u8 = 8 bits = 1 byte
    // Bump seed for the state PDA - needed to validate the correct state account
    pub state_bump: u8,
}

// Space calculation for rent determination
// Tells Anchor how much space the vault_state account needs
impl Space for VaultState {
    // The first 8 bytes are for the "Discriminator" (automatic Anchor account type identifier)
    // 1 byte for vault_bump, 1 byte for state_bump
    const INIT_SPACE: usize = 8 + 1 + 1;
}

// ------------------------------------------------------------------------------------------
// PAYMENT OPERATIONS
// ------------------------------------------------------------------------------------------

// Payment accounts - Used for both deposit and withdraw instructions
// Defines the accounts needed for transferring funds to/from the vault
#[derive(Accounts)]
pub struct Payment<'info> {
    // User account - must sign the transaction
    // 'mut' because their SOL balance will change (sending or receiving funds)
    #[account(mut)]
    pub user: Signer<'info>,

    // The vault_state PDA - not marked 'mut' since data isn't changing during payments
    // We use this to verify ownership and get the bump seeds for the vault
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    // The vault PDA - marked 'mut' because its SOL balance will change
    // This is where the funds are actually stored
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    // Required for transferring SOL
    pub system_program: Program<'info, System>,
}

// ------------------------------------------------------------------------------------------
// DEPOSIT IMPLEMENTATION
// ------------------------------------------------------------------------------------------

impl<'info> Payment<'info> {
    // Deposits SOL from the user's wallet to their vault
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // Get references to the System Program (which handles SOL transfers)
        let cpi_program = self.system_program.to_account_info();

        // Set up the accounts for the transfer
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(), // Source: user's wallet
            to: self.vault.to_account_info(),  // Destination: vault PDA
        };

        // Create the CPI (Cross-Program Invocation) context
        // No signer seeds needed because the user is the signer
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute the transfer
        transfer(cpi_ctx, amount)
    }
}

// ------------------------------------------------------------------------------------------
// WITHDRAW IMPLEMENTATION
// ------------------------------------------------------------------------------------------

impl<'info> Payment<'info> {
    // Withdraws SOL from the vault back to the user's wallet
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        // Get references to the System Program
        let cpi_program = self.system_program.to_account_info();

        // Set up the accounts for the transfer, now in reverse direction
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(), // Source: vault PDA
            to: self.user.to_account_info(),    // Destination: user's wallet
        };

        // Create the seeds for PDA signing
        // The vault PDA must "sign" this transaction since it's sending funds
        let key_ref = self.user.key();
        let vault_bump = self.vault_state.vault_bump;
        let seeds = &[b"vault", key_ref.as_ref(), &[vault_bump]];
        let signer_seeds = &[&seeds[..]];

        // Create the CPI context with signer seeds to allow the PDA to "sign"
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Execute the transfer
        transfer(cpi_ctx, amount)
    }
}

// ------------------------------------------------------------------------------------------
// CLOSE VAULT IMPLEMENTATION
// ------------------------------------------------------------------------------------------

// CloseVault accounts - Used for the close_vault instruction
#[derive(Accounts)]
pub struct CloseVault<'info> {
    // User account - must sign the transaction
    // 'mut' because they will receive funds from closed accounts
    #[account(mut)]
    pub user: Signer<'info>,

    // The vault_state PDA - marked 'mut' because it will be closed
    // The 'close = user' constraint automatically closes this account and
    // sends its rent-exempt balance to the user
    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user
    )]
    pub vault_state: Account<'info, VaultState>,

    // The vault PDA - marked 'mut' because funds will be withdrawn
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    // Required for transferring SOL
    pub system_program: Program<'info, System>,
}

impl<'info> CloseVault<'info> {
    // Closes the vault, returning all funds to the user
    pub fn close(&mut self) -> Result<()> {
        // Get the current balance of the vault
        let vault_balance = self.vault.to_account_info().lamports();

        // Only attempt a transfer if the vault has funds
        if vault_balance > 0 {
            // Get references to the System Program
            let cpi_program = self.system_program.to_account_info();

            // Set up the accounts for the transfer
            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.user.to_account_info(),
            };

            // Create the seeds for PDA signing
            let key_ref = self.user.key();
            let vault_bump = self.vault_state.vault_bump;
            let seeds = &[b"vault", key_ref.as_ref(), &[vault_bump]];
            let signer_seeds = &[&seeds[..]];

            // Create the CPI context with signer seeds
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            // Transfer all remaining funds from vault to user
            transfer(cpi_ctx, vault_balance)?;
        }

        // The vault_state account is automatically closed due to the close = user constraint
        // This transfers the rent-exempt SOL back to the user
        // The vault PDA itself (being a system account) cannot be closed, but its balance is now zero

        Ok(())
    }
}
