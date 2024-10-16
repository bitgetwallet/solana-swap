use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(mut, seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(address = admin_info.authority)]
    pub authority: Signer<'info>,
}

pub fn set_authority(ctx: Context<SetAuthority>, authority: Pubkey) -> Result<()> {
    let admin_info = &mut ctx.accounts.admin_info;

    admin_info.authority = authority;
    Ok(())
}
