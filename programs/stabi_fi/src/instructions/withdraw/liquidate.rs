use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{calculate_health_factor, error::ErrorCode, get_usd_value, Collateral, Config};

use super::{burn_tokens, withdraw_sols};

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    pub price_update: Account<'info, PriceUpdateV2>,
    #[account(
        seeds = [b"config"],
        bump = config_account.bump,
        has_one = mint_account
    )]
    pub config_account: Account<'info, Config>,
    #[account(
        mut,
        has_one = sol_account
    )]
    pub collateral_account: Account<'info, Collateral>,
    #[account(mut)]
    pub sol_account: SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = liquidator,
        associated_token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn liquidate_handler(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
    let health_factor = calculate_health_factor(
        &ctx.accounts.price_update,
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
    )?;

    require!(
        health_factor < ctx.accounts.config_account.min_health_factor,
        ErrorCode::AboveHealthFactor
    );

    let lamports = get_usd_value(&ctx.accounts.price_update, &amount_to_burn)?;
    let liquidation_bonus = lamports * ctx.accounts.config_account.liquidation_bonus / 100;
    let amount_to_liquidate = lamports + liquidation_bonus;

    withdraw_sols(
        &ctx.accounts.sol_account,
        &ctx.accounts.liquidator.to_account_info(),
        &ctx.accounts.system_program,
        &ctx.accounts.collateral_account.depositor,
        ctx.accounts.collateral_account.bump_sol_account,
        amount_to_liquidate,
    )?;

    burn_tokens(
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.liquidator,
        &ctx.accounts.token_program,
        amount_to_burn,
    )?;

    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports();
    collateral_account.amount_minted -= amount_to_burn;

    Ok(())
}
