use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Collateral{
    pub depositor: Pubkey,
    pub sol_account: Pubkey,
    pub token_account: Pubkey,
    pub lamport_balance: u64,
    pub amount_minted: u64,
    pub bump: u8,
    pub bump_sol_account: u8,
    pub is_initialized: bool,
}