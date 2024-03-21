use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SetIsPaused<'info> {
    #[account(mut)]
    pub bkswap_account: Account<'info, Bkswap>,

    #[account(address = bkswap_account.authority)]
    pub authority: Signer<'info>,
}

pub fn set_is_paused(ctx: Context<SetIsPaused>, is_paused: bool) -> Result<()> {
    let bkswap_account = &mut ctx.accounts.bkswap_account;

    bkswap_account.is_paused = is_paused;
    Ok(())
}
