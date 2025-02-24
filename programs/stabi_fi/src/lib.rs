pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AbjnBAfwWjLNWEYSdAwsMyw1rECnUwbPMW9gEdgjZSog");

#[program]
pub mod stabi_fi {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeConfig>,
        liquidation_threshold: u64,
        liquidation_bonus: u64,
        min_health_factor: u64,
    ) -> Result<()> {
        initialize_config_handler(
            ctx,
            liquidation_threshold,
            liquidation_bonus,
            min_health_factor,
        )?;
        Ok(())
    }

    pub fn deposit_collateral_and_mint_stable_tokens(
        ctx: Context<DepositCollateralAndMintTokens>,
        amount_to_mint: u64,
        amount_collateral: u64,
    ) -> Result<()> {
        deposit_collateral_and_mint_tokens_handler(ctx, amount_to_mint, amount_collateral)?;
        Ok(())
    }
}
