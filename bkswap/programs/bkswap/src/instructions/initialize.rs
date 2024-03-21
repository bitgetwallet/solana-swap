use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = funder, space = 200)]
    pub bkswap_account: Account<'info, Bkswap>,

    #[account(mut)]
    pub funder: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize(
    ctx: Context<Initialize>,
    authority: Pubkey,
    fee_receiver: Pubkey,
    fee_rate: u16,
) -> Result<()> {
    let bkswap_account = &mut ctx.accounts.bkswap_account;

    bkswap_account.authority = authority;
    bkswap_account.fee_receiver = fee_receiver;
    bkswap_account.fee_rate = fee_rate;
    Ok(())
}
