use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct SetIsPaused<'info> {
    #[account(mut,seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(address = admin_info.operator)]
    pub operator: Signer<'info>,
}

pub fn set_is_paused(ctx: Context<SetIsPaused>, is_paused: bool) -> Result<()> {
    let account = &mut ctx.accounts.admin_info;
    require!(is_paused != account.is_paused, ErrorCode::InvalidInputParam);
    msg!("old paused is {:?}", account.is_paused);
    account.is_paused = is_paused;
    msg!("new paused is {:?}", account.is_paused);

    Ok(())
}
