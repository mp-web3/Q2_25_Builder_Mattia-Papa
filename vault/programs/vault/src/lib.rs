#[allow(unexpected_cfgs)]
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
declare_id!("2FmfXbj5gPvLD3vjmKKHMsqo14K3VvzxWcJ1zD5pZ8G9");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        ctx.accounts.close()
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>, // This is like a wallet, but in this case is a PDA where the funds will be stored
    pub system_program: Program<'info, System>, // This is the system_program that gets executes
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}

#[account]
pub struct VaultState {
    pub vault_bump: u8, // u8 = 8 bites = 1 byte
    pub state_bump: u8,
}

impl Space for VaultState {
    // The first 8 bytes are for the "Discriminator", the 1 is for 8 bits of vault_bump, and the
    // last 1 is for the 8 bits of state_bump.
    const INIT_SPACE: usize = 8 + 1 + 1;
}

///////////////////////////

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", user.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    // We need system program to change the balance
    pub system_program: Program<'info, System>,
}

//////////////////////// DEPOSIT ////////////////////////////

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)
    }
}

//////////////////////// WITHDRAW ////////////////////////////

impl<'info> Payment<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let key_ref = self.user.key();
        let vault_bump = self.vault_state.vault_bump;
        let seeds = &[b"vault", key_ref.as_ref(), &[vault_bump]];
        let signer_seeds = &[&seeds[..]];

        // To withdraw the PDA has to sign
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)
    }
}

//////////////////////// CLOSE VAULT ////////////////////////////

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CloseVault<'info> {
    pub fn close(&mut self) -> Result<()> {
        // Get the balance of the vault
        let vault_balance = self.vault.to_account_info().lamports();

        // Transfer remaining funds to the user
        if vault_balance > 0 {
            let cpi_program = self.system_program.to_account_info();
            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.user.to_account_info(),
            };

            let key_ref = self.user.key();
            let vault_bump = self.vault_state.vault_bump;
            let seeds = &[b"vault", key_ref.as_ref(), &[vault_bump]];
            let signer_seeds = &[&seeds[..]];

            // Transfer all funds from vault to user
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            transfer(cpi_ctx, vault_balance)?;
        }

        // The vault_state account is automatically closed due to the close = user constraint
        // This transfers the rent-exempt SOL back to the user

        Ok(())
    }
}
