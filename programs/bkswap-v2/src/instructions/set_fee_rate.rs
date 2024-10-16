use anchor_lang::prelude::*;

use crate::state::*;
use crate::consts::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct SetFeeRate<'info> {
    #[account(mut, seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(address = admin_info.authority)]
    pub authority: Signer<'info>,
}

pub fn set_fee_rate(ctx: Context<SetFeeRate>, fee_rate: u16) -> Result<()> {
    require!(fee_rate <= MAX_PROTOCOL_FEE_RATE, ErrorCode::FeeRateTooHigh);

    let admin_info = &mut ctx.accounts.admin_info;

    admin_info.fee_rate = fee_rate;
    Ok(())
}

