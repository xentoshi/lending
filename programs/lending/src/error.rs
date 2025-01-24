use anchor_lang::prelude::*;

#[error_code]
pub enum LendingError {
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Requested amount is greater than borrowable amount")]
    InsufficientCollateral,
    #[msg("Over repay")]
    OverRepay,
}