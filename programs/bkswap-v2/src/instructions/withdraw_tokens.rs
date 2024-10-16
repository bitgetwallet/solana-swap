use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct WithdrawOtherToken<'info> {
    #[account(mut,seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(address = admin_info.operator)]
    pub operator: Signer<'info>,
    /// The mint to distribute.
    pub mint: Account<'info, Mint>,// other token address
    #[account(mut, token::mint=mint)]
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint=mint,
        token::authority=admin_info.receiver
    )]
    pub to_token_account: Account<'info, TokenAccount>,
    #[account(address = from_token_account.owner)]
    pub from_ata_owner: Signer<'info>,// this program Id
    /// SPL [Token] program.
    pub token_program: Program<'info, Token>,
}

pub fn withdraw_other_token(ctx: Context<WithdrawOtherToken>, amount: u64) -> Result<()> {

    require!(amount <= ctx.accounts.from_token_account.amount, ErrorCode::AmountOverBalance);
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.from_token_account.to_account_info(),
                to: ctx.accounts.to_token_account.to_account_info(),
                authority: ctx.accounts.from_ata_owner.to_account_info(),
            }
        ),
        amount
    )?;

    msg!("other_token is {:?}", ctx.accounts.mint);
    msg!("amount is {:?}", amount);

    Ok(())
}
