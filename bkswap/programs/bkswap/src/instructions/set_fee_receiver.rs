use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SetFeeReceiver<'info> {
    #[account(mut)]
    pub bkswap_account: Account<'info, Bkswap>,

    #[account(address = bkswap_account.authority)]
    pub authority: Signer<'info>,
}

pub fn set_fee_receiver(ctx: Context<SetFeeReceiver>, fee_receiver: Pubkey) -> Result<()> {
    let bkswap_account = &mut ctx.accounts.bkswap_account;

    bkswap_account.fee_receiver = fee_receiver;
    Ok(())
}
