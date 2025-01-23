use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    // Owner of the user
    pub owner: Pubkey,
    // Deposited SOL
    pub deposited_sol: u64,
    // Deposited SOL shares
    pub deposited_sol_shares: u64,
    // Borrowed SOL
    pub borrowed_sol: u64,
    // Borrowed SOL shares
    pub borrowed_sol_shares: u64,
    // Deposited USDC
    pub deposited_usdc: u64,
    // Deposited USDC shares
    pub deposited_usdc_shares: u64,
    // Borrowed USDC
    pub borrowed_usdc: u64,
    // Borrowed USDC shares
    pub borrowed_usdc_shares: u64,
    // USDC mint address
    pub usdc_address: Pubkey,
    // Last updated timestamp
    pub last_updated: i64,
}


#[account]
#[derive(InitSpace)]
pub struct Bank {
    // Authority to make changes to the bank
    pub authority: Pubkey,
    // Mint address of the asset
    pub mint_address: Pubkey,
    // Current number of tokens in the bank
    pub total_deposits: u64,
    // Current number of deposit shares in the bank
    pub total_deposit_shares: u64,
    // Current number of borrowed tokens in the bank
    pub total_borrowed: u64,    
    // Current number of borrowed shares in the bank
    pub total_borrowed_shares: u64,
    // LTV at which the loan is defined as under collateral 
    pub liquidation_threshold: u64,
    // Bonus percentage of collateral that can be liquidated
    pub liquidation_bonus: u64,
    // Percentage of the collateral that can be liquidated
    pub liquidation_close_factor: u64,
    // Max percentage of the collateral that can be borrowed
    pub max_ltv: u64,
    // Last updated timestamp
    pub last_updated: i64,
    // Interest rate per second
    pub interest_rate: u64,
}