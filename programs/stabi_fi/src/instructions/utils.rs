use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{error::ErrorCode, MAX_PRICE_FEED_AGE, SOL_USD_PRICE_FEED_HEX};

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
