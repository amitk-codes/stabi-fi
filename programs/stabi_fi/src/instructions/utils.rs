use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{error::ErrorCode, Collateral, Config, MAX_PRICE_FEED_AGE, SOL_USD_PRICE_FEED_HEX};

pub fn check_health_factor(
    price_update_account: &Account<PriceUpdateV2>,
    collateral_account: &Account<Collateral>,
    config_account: &Account<Config>
) -> Result<()>{
    let collateral_value_in_usd = get_usd_value(price_update_account, &collateral_account.lamport_balance)?;

    let collateral_value_with_threshold_consideration = (collateral_value_in_usd * config_account.liquidation_threshold) / 100;

    let health_factor = collateral_value_with_threshold_consideration / collateral_account.amount_minted;

    require!(health_factor >= config_account.min_health_factor, ErrorCode::BelowHealthFactor);
    Ok(())
}

pub fn get_usd_value(
    price_update_account: &Account<PriceUpdateV2>,
    amount_in_lamports: &u64,
) -> Result<u64> {
    let sol_usd_price_feed_id = get_feed_id_from_hex(SOL_USD_PRICE_FEED_HEX)?;
    let price_response = price_update_account.get_price_no_older_than(
        &Clock::get()?,
        MAX_PRICE_FEED_AGE,
        &sol_usd_price_feed_id,
    )?;

    require!(price_response.price > 0, ErrorCode::InvalidPrice);

    // pyth generates the price in 8 decimal, so to make it 9 decimal, we have to multiply it with 10
    let price_in_usd = price_response.price as u128 * 10;

    let amount_in_usd = (*amount_in_lamports as u128 * price_in_usd) / LAMPORTS_PER_SOL as u128;

    Ok(amount_in_usd as u64)
}
