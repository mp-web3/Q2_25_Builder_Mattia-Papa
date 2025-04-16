use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError {
    #[msg("Max Stake Reached")]
    MaxStakeReached,

    #[msg("Freeze Period")]
    FreezePeriodNotPassed,
}
