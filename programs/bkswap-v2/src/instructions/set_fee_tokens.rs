use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct SetFeeTokens<'info> {
    #[account(
        seeds=[b"admin_info"],
        bump
    )]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(
        mut,
        seeds=[b"fee_tokens"],
        bump
    )]
    pub fee_tokens: Account<'info, FeeTokens>,

    #[account(mut, address = admin_info.authority)]
    pub authority: Signer<'info>
}

pub fn set_fee_tokens(
    ctx: Context<SetFeeTokens>,
    special_tokens_01: [Pubkey; 10],
    special_tokens_02: [Pubkey; 10],
    token_num: u16
) -> Result<()> {
    
    let fee_tokens = &mut ctx.accounts.fee_tokens;
    msg!("old fee_tokens.special_tokens_01 is {:?}", fee_tokens.special_tokens_01);
    fee_tokens.special_tokens_01 = special_tokens_01;
    msg!("new fee_tokens.special_tokens_01 is {:?}", fee_tokens.special_tokens_01);

    msg!("old fee_tokens.special_tokens_02 is {:?}", fee_tokens.special_tokens_02);
    fee_tokens.special_tokens_02 = special_tokens_02;
    msg!("new fee_tokens.special_tokens_02 is {:?}", fee_tokens.special_tokens_02);

    fee_tokens.total_num = token_num;
    Ok(())
}
