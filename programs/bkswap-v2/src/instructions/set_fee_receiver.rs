use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SetFeeReceiver<'info> {
    #[account(mut, seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(address = admin_info.authority)]
    pub authority: Signer<'info>,

    #[account(seeds=[b"fee_receivers"], bump)]
    pub fee_receivers: Account<'info, FeeReceivers>,
}

pub fn set_stable_token_receiver(ctx: Context<SetFeeReceiver>, new_stable_token_receiver: Pubkey) -> Result<()> {
    let fee_receivers_pda = &mut ctx.accounts.fee_receivers;

    fee_receivers_pda.stable_token_receiver = new_stable_token_receiver;
    Ok(())
}

pub fn set_other_token_receiver(ctx: Context<SetFeeReceiver>, new_other_token_receiver: Pubkey) -> Result<()> {
    let fee_receivers_pda = &mut ctx.accounts.fee_receivers;

    fee_receivers_pda.other_token_receiver = new_other_token_receiver;
    Ok(())
}
