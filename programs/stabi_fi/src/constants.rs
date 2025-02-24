use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const ANCHOR_DISCRIMINATOR: usize = 8;

pub const SOL_USD_PRICE_FEED_HEX: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
pub const MAX_PRICE_FEED_AGE: u64 = 100;