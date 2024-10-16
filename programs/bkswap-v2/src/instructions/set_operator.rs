use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SetOperator<'info> {
    #[account(mut, seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(address = admin_info.authority)]
    pub authority: Signer<'info>,
}

pub fn set_operator(ctx: Context<SetOperator>, new_operator: Pubkey) -> Result<()> {
    let admin_info = &mut ctx.accounts.admin_info;

    admin_info.operator = new_operator;
    Ok(())
}
