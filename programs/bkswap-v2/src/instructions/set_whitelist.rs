use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct SetWhitelist<'info> {
    #[account(
        seeds=[b"admin_info"],
        bump
    )]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(
        mut,
        seeds=[b"whitelist"],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,

    #[account(mut, address = admin_info.authority)]
    pub authority: Signer<'info>
}

pub fn set_whitelist(
    ctx: Context<SetWhitelist>,
    whitelist_users: [Pubkey; 10],
    user_num: u16
) -> Result<()> {
    
    let whitelist = &mut ctx.accounts.whitelist;
    msg!("old whitelist.users is {:?}", whitelist.users);
    whitelist.users = whitelist_users;
    whitelist.total_num = user_num;

    msg!("new whitelist.users is {:?}", whitelist.users);
    Ok(())
}
