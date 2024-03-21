use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(mut)]
    pub bkswap_account: Account<'info, Bkswap>,

    #[account(address = bkswap_account.authority)]
    pub authority: Signer<'info>,
}

pub fn set_authority(ctx: Context<SetAuthority>, authority: Pubkey) -> Result<()> {
    let bkswap_account = &mut ctx.accounts.bkswap_account;

    bkswap_account.authority = authority;
    Ok(())
}
