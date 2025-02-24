use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token_2022::{mint_to, MintTo};
use anchor_spl::token_interface::{Mint, TokenAccount, Token2022};

pub fn deposit_sol_collateral<'info>(
    system_program: &Program<'info, System>,
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    amount: u64,
) -> Result<()> {
    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
            },
        ),
        amount,
    )?;
    Ok(())
}

pub fn mint_stable_coins<'info>(
    bump: u8,
    token_program: &Program<'info, Token2022>,
    mint_account: &InterfaceAccount<'info, Mint>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: u64,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[bump]]];
    mint_to(
        CpiContext::new(
            token_program.to_account_info(),
            MintTo {
                mint: mint_account.to_account_info(),
                to: to.to_account_info(),
                authority: mint_account.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        amount,
    )?;
    Ok(())
}
