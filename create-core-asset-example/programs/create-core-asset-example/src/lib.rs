use anchor_lang::prelude::*;

use mpl_core::{
    ID as MPL_CORE_ID,
    accounts::BaseCollectionV1, 
    instructions::CreateV2CpiBuilder, 
};

declare_id!("Atj9w75qkzaYXi6595sABKLaVYaUTHdLcgs1Dw9JULvs");


#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateAssetArgs {

}

#[program]
pub mod create_core_asset_example {
    use super::*;

    pub fn create_core_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {

}
