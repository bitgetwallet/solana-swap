use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SetReceiver<'info> {
    #[account(mut, seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(address = admin_info.authority)]
    pub authority: Signer<'info>,
}

pub fn set_receiver(ctx: Context<SetReceiver>, new_receiver: Pubkey) -> Result<()> {
    let admin_info = &mut ctx.accounts.admin_info;

    admin_info.receiver = new_receiver;
    Ok(())
}
