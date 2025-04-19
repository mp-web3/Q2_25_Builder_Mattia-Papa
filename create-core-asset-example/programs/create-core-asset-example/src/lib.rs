use anchor_lang::prelude::*;

use mpl_core::{
    ID as MPL_CORE_ID,
    accounts::BaseCollectionV1, 
    instructions::CreateV2CpiBuilder, 
};

declare_id!("Atj9w75qkzaYXi6595sABKLaVYaUTHdLcgs1Dw9JULvs");

#[program]
pub mod create_core_asset_example {
    use super::*;

    pub fn create_core_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {

        // Some of the accounts in the CreateAsset struct are optional, 
        // meaning their value could be either Some(account) or None. 
        // To handle these optional accounts before passing them to the builder, 
        // we use a match statement that allows us to check 
        // if an account is present (Some) or absent (None) and based on this check,
        // we bind the account as Some(account.to_account_info())
        let collection = match &ctx.accounts.collection {
          Some(collection) => Some(collection.to_account_info()),
          None => None,
        };
      
        let authority = match &ctx.accounts.authority {
          Some(authority) => Some(authority.to_account_info()),
          None => None,
        };
      
        let owner = match &ctx.accounts.owner {
          Some(owner) => Some(owner.to_account_info()),
          None => None,
        };
      
        let update_authority = match &ctx.accounts.update_authority {
          Some(update_authority) => Some(update_authority.to_account_info()),
          None => None,
        };
        
        // After preparing all the necessary accounts, we pass them to the CreateV2CpiBuilder and 
        // use .invoke() to execute the instruction, or .invoke_signed() if we need to use signer seeds.
        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
          .asset(&ctx.accounts.asset.to_account_info())
          .collection(collection.as_ref())
          .authority(authority.as_ref())
          .payer(&ctx.accounts.payer.to_account_info())
          .owner(owner.as_ref())
          .update_authority(update_authority.as_ref())
          .system_program(&ctx.accounts.system_program.to_account_info())
          .name(args.name)
          .uri(args.uri)
          .invoke()?;
      
        Ok(())
    }
}

// To keep our function organized and avoid clutter from too many parameters, 
// it's standard practice to pass all inputs through a structured format. 
// This is achieved by defining an argument struct (CreateAssetArgs) and deriving AnchorDeserialize and AnchorSerialize, 
// which allows the struct to be serialized into a binary format using NBOR, and making it readable by Anchor.
#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateAssetArgs {
    name: String,
    uri: string,
}

// Some of this accounts are optional
#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub asset: Signer<'info>,
    #[account(mut)]
    pub collection: Option<Account<'info, BaseCollectionV1>>,
    pub authority: Option<Signer<'info>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: this account will be checked by the mpl_core program
    pub owner: Option<UncheckedAccount<'info>>,
    /// CHECK: this account will be checked by the mpl_core program
    pub update_authority: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_ID)]
    /// CHECK: this account is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}
