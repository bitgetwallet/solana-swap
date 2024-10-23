use anchor_lang::prelude::*;
use crate::consts::*;
use crate::state::*;
use crate::errors::ErrorCode;

use anchor_spl::{
    token_interface::{Mint, TokenAccount},
    token::{self, Token},
    token_2022::{self}
  };

#[derive(Accounts, Clone)]
pub struct CollectFee<'info> {
    /// CHECK: Safe
    #[account(mut, seeds=[b"admin_info"], bump)]
    pub admin_info: Account<'info, AdminInfo>,
    /// CHECK: Safe
    #[account(
        mut, 
        constraint = fee_to_token_account.mint == mint.clone().expect("Mint is none").key()
    )]
    pub fee_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: Safe
    #[account(mut, constraint = user_source_token_account.mint == mint.clone().expect("Mint is none").key())]
    pub user_source_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub user_owner: Signer<'info>,

    pub mint: Option<Box<InterfaceAccount<'info, Mint>>>,

    /// SPL program for token transfers
    /// CHECK: Safe
    pub token_program: AccountInfo<'info>,

    /// SPL program 2022 for token transfers
    pub token_program_2022: Option<AccountInfo<'info>>
}

pub fn collect_fee(
    ctx: Context<CollectFee>,
    amount: u64,
) -> Result<u64> {
    let admin_info = &mut ctx.accounts.admin_info;
    require!(!admin_info.is_paused, ErrorCode::ProtocolPaused);
    require!(ctx.accounts.user_owner.key() != Pubkey::default(), ErrorCode::UserCannotBeZeroAddress);
    require!(amount > 0 , ErrorCode::AmountCannotBeZero);

    if ctx.accounts.admin_info.users.contains(&ctx.accounts.user_owner.key()) {
        return Ok(amount);
    }
    
    if 
        ctx.accounts.admin_info.special_tokens_01.contains(&ctx.accounts.mint.clone().expect("MintIsNone").key()) 
        || ctx.accounts.admin_info.special_tokens_02.contains(&ctx.accounts.mint.clone().expect("MintIsNone").key()) 
    {
        require!(ctx.accounts.fee_to_token_account.owner.key() == ctx.accounts.admin_info.stable_token_receiver, ErrorCode::InputFeeReceiverIsInvalid);
    } else {
        require!(ctx.accounts.fee_to_token_account.owner.key() == ctx.accounts.admin_info.other_token_receiver, ErrorCode::InputFeeReceiverIsInvalid);
    }

    let fee_amount: u64 = ((amount as u128) * (ctx.accounts.admin_info.fee_rate as u128) / PROTOCOL_FEE_RATE_MUL_VALUE).try_into().unwrap();

    let mut token_program_info = ctx.accounts.token_program.to_account_info();
    let user_ata_info = ctx.accounts.user_source_token_account.to_account_info();
    let mint = &ctx.accounts.mint;
    let token_program_2022 = &ctx.accounts.token_program_2022;

    match (mint, token_program_2022) {
        (Some(mint), Some(token_program_2022)) => {
            if user_ata_info.owner == token_program_2022.key {
                token_program_info = token_program_2022.to_account_info()
            }
            token_2022::transfer_checked(
                CpiContext::new(
                    token_program_info,
                    token_2022::TransferChecked {
                        from: user_ata_info,
                        to: ctx.accounts.fee_to_token_account.to_account_info(),
                        authority: ctx.accounts.user_owner.to_account_info(),
                        mint: mint.to_account_info(),
                    },
                ),
                fee_amount,
                mint.decimals,
            )?;
        }
        _ => token::transfer(
            CpiContext::new(
                token_program_info,
                token::Transfer {
                    from: user_ata_info,
                    to: ctx.accounts.fee_to_token_account.to_account_info(),
                    authority: ctx.accounts.user_owner.to_account_info(),
                },
            ),
            fee_amount,
        )?,
    };

    Ok(amount - fee_amount)
}
