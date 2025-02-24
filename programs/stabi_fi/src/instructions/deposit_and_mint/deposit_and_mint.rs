use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, Token2022},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{check_health_factor, Collateral, Config, ANCHOR_DISCRIMINATOR};

use super::{deposit_sol_collateral, mint_stable_coins};

#[derive(Accounts)]
pub struct DepositCollateralAndMintTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [b"config"],
        bump = config_account.bump,
        has_one = mint_account
    )]
    pub config_account: Box<Account<'info, Config>>,

    #[account(
        init_if_needed,
        payer = depositor,
        space = ANCHOR_DISCRIMINATOR + Collateral::INIT_SPACE,
        seeds = [b"collateral", depositor.key().as_ref()],
        bump
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(
        mut,
        seeds = [b"sol", depositor.key.as_ref()],
        bump
    )]
    pub sol_account: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_account,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn deposit_collateral_and_mint_tokens_handler(
    ctx: Context<DepositCollateralAndMintTokens>,
    amount_to_mint: u64,
    amount_collateral: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() + amount_collateral;
    collateral_account.amount_minted += amount_to_mint;


    if !collateral_account.is_initialized {
        collateral_account.is_initialized = true;
        collateral_account.depositor = ctx.accounts.depositor.key();
        collateral_account.sol_account = ctx.accounts.sol_account.key();
        collateral_account.token_account = ctx.accounts.token_account.key();
        collateral_account.bump = ctx.bumps.collateral_account;
        collateral_account.bump_sol_account = ctx.bumps.sol_account;
    }


    // checking the health factor
    check_health_factor(
        &ctx.accounts.price_update,
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
    )?;

    // depositing the collateral
    deposit_sol_collateral(
        &ctx.accounts.system_program,
        &ctx.accounts.depositor,
        &ctx.accounts.sol_account,
        amount_collateral,
    )?;

    // minting the stable coins

    mint_stable_coins(
        ctx.accounts.config_account.bump_mint_account,
        &ctx.accounts.token_program,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        amount_to_mint,
    )?;

    Ok(())
}
