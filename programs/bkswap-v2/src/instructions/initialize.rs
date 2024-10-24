use anchor_lang::prelude::*;
use crate::state::*;
use crate::program::Bkswapv2;
use crate::errors::ErrorCode;
use crate::consts::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, payer = authority, 
        space = 8 + AdminInfo::LEN,
        seeds=[b"admin_info"],
        bump
    )]
    pub admin_info: Box<Account<'info, AdminInfo>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
    pub program: Program<'info, Bkswapv2>,
    #[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))]
    pub program_data: Box<Account<'info, ProgramData>>,
    
    system_program: Program<'info, System>,
}

pub fn initialize(
    ctx: Context<Initialize>,
    authority: Pubkey,
    operator: Pubkey,
    receiver: Pubkey,
    stable_token_receiver: Pubkey,
    other_token_receiver: Pubkey,
    fee_rate: u16,
    whitelist_users: [Pubkey; 10],
    user_num: u16
) -> Result<()> {
    require!(authority != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    require!(operator != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    require!(receiver != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    require!(stable_token_receiver != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    require!(other_token_receiver != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    require!(fee_rate <= MAX_PROTOCOL_FEE_RATE, ErrorCode::FeeRateTooHigh);
    require!(user_num <= 10, ErrorCode::UserNumTooMany);

    let admin_info = &mut ctx.accounts.admin_info;

    admin_info.authority = authority;
    admin_info.operator = operator;
    admin_info.receiver = receiver;

    admin_info.stable_token_receiver = stable_token_receiver;
    admin_info.other_token_receiver = other_token_receiver;
    admin_info.fee_rate = fee_rate;

    admin_info.users = whitelist_users;
    admin_info.real_users_num = user_num;

    msg!("authority is {:?}", admin_info.authority);
    msg!("operator is {:?}", admin_info.operator);
    msg!("receiver is {:?}", admin_info.receiver);

    msg!("stable_token_receiver is {:?}", admin_info.stable_token_receiver);
    msg!("other_token_receiver is {:?}", admin_info.other_token_receiver);

    msg!("fee_rate is {:?}", admin_info.fee_rate);
    msg!("whitelist.users is {:?}", admin_info.users);
    msg!("real_users_num is {:?}", admin_info.real_users_num);
    Ok(())
}
