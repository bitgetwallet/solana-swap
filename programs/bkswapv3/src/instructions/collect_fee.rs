use anchor_lang::prelude::*;
use solana_program::pubkey;
use crate::consts::*;
use crate::state::*;
use crate::errors::ErrorCode;

use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
    token::{self},
    token_2022::{self}
};

pub const TOKEN_PROGRAM_2022: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

#[derive(Accounts, Clone)]
pub struct CollectFee<'info> {
    /// CHECK: Safe
    #[account(mut, seeds=[b"admin_info"], bump)]
    pub admin_info: Box<Account<'info, AdminInfo>>,

    #[account(mut, constraint = fee_to_token_account.mint == mint.key())]
    pub fee_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut, constraint = prededuct_to_token_account.mint == mint.key())]
    pub prededuct_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut, constraint = user_source_token_account.mint == mint.key())]
    pub user_source_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub user_owner: Signer<'info>,

    #[account( mint::token_program = token_program_x)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: Safe
    #[account(
        constraint = (token_program_x.key() == spl_token::ID) || (token_program_x.key() == TOKEN_PROGRAM_2022.key())
    )]
    pub token_program_x: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

pub fn collect_fee(
    ctx: Context<CollectFee>,
    amount: u64,
    prededuct_amount: u64,
    fee_rate: u16
) -> Result<u64> {
    let admin_info = &mut ctx.accounts.admin_info;
    require!(!admin_info.is_paused, ErrorCode::ProtocolPaused); 
    require!(ctx.accounts.user_owner.key() != Pubkey::default(), ErrorCode::UserCannotBeZeroAddress);
    require!(amount > 0 , ErrorCode::AmountCannotBeZero);
    require!(amount + prededuct_amount <= ctx.accounts.user_source_token_account.amount, ErrorCode::AmountOverBalance);

    require!(
        admin_info.min_fee_rate_limit <= fee_rate && fee_rate <= admin_info.max_fee_rate_limit, ErrorCode::FeeRateTooLowOrTooHigh
    );

    // check fee_to_token_account.owner
    if 
        ctx.accounts.admin_info.special_tokens_01.contains(&ctx.accounts.mint.key()) 
        || ctx.accounts.admin_info.special_tokens_02.contains(&ctx.accounts.mint.key()) 
    {
        require!(ctx.accounts.fee_to_token_account.owner.key() == ctx.accounts.admin_info.stable_token_receiver, ErrorCode::InputFeeReceiverIsInvalid);
    } else {
        require!(ctx.accounts.fee_to_token_account.owner.key() == ctx.accounts.admin_info.other_token_receiver, ErrorCode::InputFeeReceiverIsInvalid);
    }

    // check prededuct_to_token_account.owner
    require!(ctx.accounts.admin_info.prededuct_receivers.contains(&ctx.accounts.prededuct_to_token_account.owner.key()), ErrorCode::InvalidPredeductReceiver);

    let token_program_info = ctx.accounts.token_program_x.to_account_info();
    let user_ata_info = ctx.accounts.user_source_token_account.to_account_info();
    let mint = ctx.accounts.mint.to_account_info();
    let mint_ptr = &ctx.accounts.mint;
    let user_owner_info = ctx.accounts.user_owner.to_account_info();
    let user_ata_owner = *user_ata_info.owner;

    // Collect all transfer amounts and destinations
    let mut transfer_amounts = vec![prededuct_amount];
    let mut transfer_destinations = vec![ctx.accounts.prededuct_to_token_account.to_account_info()];

    let mut fee_amount: u64 = ((amount as u128) * (fee_rate as u128) / PROTOCOL_FEE_RATE_MUL_VALUE).try_into().unwrap();
    if !ctx.accounts.admin_info.users.contains(&ctx.accounts.user_owner.key()) {
        
        transfer_amounts.push(fee_amount);
        transfer_destinations.push(ctx.accounts.fee_to_token_account.to_account_info());
        
    } else {
        fee_amount = 0u64;
    }

    // Perform transfers
    for (amount, destination) in transfer_amounts.iter().zip(transfer_destinations.iter()) {
        if user_ata_owner == TOKEN_PROGRAM_2022 {
            token_2022::transfer_checked(
                CpiContext::new(
                    token_program_info.clone(),
                    token_2022::TransferChecked {
                        from: user_ata_info.clone(),
                        to: destination.clone(),
                        authority: user_owner_info.clone(),
                        mint: mint.clone(),
                    },
                ),
                *amount,
                mint_ptr.decimals,
            )?;
        } else {
            token::transfer(
                CpiContext::new(
                    token_program_info.clone(),
                    token::Transfer {
                        from: user_ata_info.clone(),
                        to: destination.clone(),
                        authority: user_owner_info.clone(),
                    },
                ),
                *amount,
            )?;
        }
    }

    Ok(amount - fee_amount)
}