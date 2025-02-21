use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub mint_account: Pubkey,
    pub bump: u8,
    pub bump_mint_account: u8,
    pub liquidation_threshold: u64,
    pub liquidation_bonus: u64,
    pub min_health_factor: u64,
}
