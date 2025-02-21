use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022};

use crate::{Config, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR + Config::INIT_SPACE,
        seeds = [b"config"],
        bump,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [b"mint"],
        bump,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
        mint::decimals = 9,
        mint::token_program = token_program
    )]
    pub mint_authority: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

pub fn initialize_config_handler(
    ctx: Context<InitializeConfig>,
    liquidation_threshold: u64,
    liquidation_bonus: u64,
    min_health_factor: u64,
) -> Result<()> {
    ctx.accounts.config_account.set_inner(Config {
        authority: ctx.accounts.authority.key(),
        mint_account: ctx.accounts.mint_authority.key(),
        bump: ctx.bumps.config_account,
        bump_mint_account: ctx.bumps.mint_authority,
        liquidation_threshold,
        liquidation_bonus,
        min_health_factor,
    });
    Ok(())
}
