use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SetIsPaused<'info> {
    #[account(mut, seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(mut, address = admin_info.operator)]
    pub operator: Signer<'info>,
}

pub fn set_is_paused(ctx: Context<SetIsPaused>, is_paused: bool) -> Result<()> {
    let admin_info = &mut ctx.accounts.admin_info;

    admin_info.is_paused = is_paused;
    Ok(())
}
