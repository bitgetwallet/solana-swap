use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount};
use crate::consts::*;
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts, Clone)]
pub struct CollectFee<'info> {
    /// CHECK: Safe
    #[account(mut)]
    pub bkswap_account: Account<'info, Bkswap>,
    /// CHECK: Safe
    #[account(mut, constraint = fee_to_token_account.owner == bkswap_account.fee_receiver)]
    pub fee_to_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: Safe
    #[account(mut)]
    pub user_source_token_account: AccountInfo<'info>,
    pub user_owner: Signer<'info>,
    /// CHECK: Safe
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

pub fn collect_fee(
    ctx: Context<CollectFee>,
    amount: u64,
) -> Result<u64> {
    require!(!ctx.accounts.bkswap_account.is_paused,ErrorCode::ProtocolPaused);

    let fee_amount: u64 = ((amount as u128) * (ctx.accounts.bkswap_account.fee_rate as u128) / PROTOCOL_FEE_RATE_MUL_VALUE).try_into().unwrap();
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.user_source_token_account.to_account_info(),
        to: ctx.accounts.fee_to_token_account.to_account_info(),
        authority: ctx.accounts.user_owner.to_account_info(),
    };
    token::transfer(
        CpiContext::new(
            ctx.accounts.spl_token_program.to_account_info(),
            cpi_accounts
        ),
        fee_amount,
    )?;
    Ok(amount - fee_amount)
}
