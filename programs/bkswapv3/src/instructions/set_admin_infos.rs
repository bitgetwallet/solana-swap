use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;
use crate::consts::*;

pub fn set_authority(ctx: Context<SetAdminInfo>, authority: Pubkey) -> Result<()> {
    require!(authority != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    let account = &mut ctx.accounts.admin_info;
    require!(authority.key() != account.authority.key(), ErrorCode::ValueCannotBeEqual);

    msg!("old authority is {:?}", account.authority);
    account.authority = authority;
    msg!("new authority is {:?}", account.authority);
    Ok(())
}

pub fn set_operator(ctx: Context<SetAdminInfo>, operator: Pubkey) -> Result<()> {
    require!(operator != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    let account = &mut ctx.accounts.admin_info;
    require!(operator.key() != account.operator.key(), ErrorCode::ValueCannotBeEqual);

    msg!("old operator is {:?}", account.operator);
    account.operator = operator;
    msg!("new operator is {:?}", account.operator);
    Ok(())
}

pub fn set_receiver(ctx: Context<SetAdminInfo>, receiver: Pubkey) -> Result<()> {
    require!(receiver != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    let account = &mut ctx.accounts.admin_info;
    require!(receiver.key() != account.receiver.key(), ErrorCode::ValueCannotBeEqual);

    msg!("old receiver is {:?}", account.receiver);
    account.receiver = receiver;
    msg!("new receiver is {:?}", account.receiver);

    Ok(())
}

pub fn set_stable_token_receiver(ctx: Context<SetAdminInfo>, new_stable_token_receiver: Pubkey) -> Result<()> {
    require!(new_stable_token_receiver != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    let admin_info = &mut ctx.accounts.admin_info;
    require!(new_stable_token_receiver.key() != admin_info.stable_token_receiver.key(), ErrorCode::ValueCannotBeEqual);

    msg!("old stable_token_receiver is {:?}", admin_info.stable_token_receiver);
    admin_info.stable_token_receiver = new_stable_token_receiver;
    msg!("new stable_token_receiver is {:?}", admin_info.stable_token_receiver);
    Ok(())
}

pub fn set_other_token_receiver(ctx: Context<SetAdminInfo>, new_other_token_receiver: Pubkey) -> Result<()> {
    require!(new_other_token_receiver != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    let admin_info = &mut ctx.accounts.admin_info;
    require!(new_other_token_receiver.key() != admin_info.other_token_receiver.key(), ErrorCode::ValueCannotBeEqual);

    msg!("old other_token_receiver is {:?}", admin_info.other_token_receiver);
    admin_info.other_token_receiver = new_other_token_receiver;
    msg!("new other_token_receiver is {:?}", admin_info.other_token_receiver);

    Ok(())
}

pub fn set_min_fee_rate_limit(ctx: Context<SetAdminInfo>, min_fee_rate_limit: u16) -> Result<()> {
    require!(min_fee_rate_limit <= MAX_PROTOCOL_FEE_RATE, ErrorCode::FeeRateTooHigh);
    
    let admin_info = &mut ctx.accounts.admin_info;
    require!(min_fee_rate_limit != admin_info.min_fee_rate_limit, ErrorCode::ValueCannotBeEqual);

    msg!("old min_fee_rate_limit is {:?}", admin_info.min_fee_rate_limit);
    admin_info.min_fee_rate_limit = min_fee_rate_limit;
    msg!("new min_fee_rate_limit is {:?}", admin_info.min_fee_rate_limit);

    Ok(())
}

pub fn set_max_fee_rate_limit(ctx: Context<SetAdminInfo>, max_fee_rate_limit: u16) -> Result<()> {
    require!(max_fee_rate_limit <= MAX_PROTOCOL_FEE_RATE, ErrorCode::FeeRateTooHigh);
    
    let admin_info = &mut ctx.accounts.admin_info;
    require!(max_fee_rate_limit != admin_info.max_fee_rate_limit, ErrorCode::ValueCannotBeEqual);

    msg!("old max_fee_rate_limit is {:?}", admin_info.max_fee_rate_limit);
    admin_info.max_fee_rate_limit = max_fee_rate_limit;
    msg!("new max_fee_rate_limit is {:?}", admin_info.max_fee_rate_limit);

    Ok(())
}

pub fn set_fee_tokens(
    ctx: Context<SetAdminInfo>,
    special_tokens: [Pubkey; 10],
    is_tokens_01:bool,
    token_num: u16
) -> Result<()> {
    require!(token_num <= 20, ErrorCode::TokenNumTooMany); 
    let admin_info = &mut ctx.accounts.admin_info;
    if is_tokens_01 {
        msg!("old special_tokens_01 is {:?}", admin_info.special_tokens_01);
        admin_info.special_tokens_01 = special_tokens;
        msg!("new special_tokens_01 is {:?}", admin_info.special_tokens_01);
    } else {
        msg!("old special_tokens_02 is {:?}", admin_info.special_tokens_02);
        admin_info.special_tokens_02 = special_tokens;
        msg!("new special_tokens_02 is {:?}", admin_info.special_tokens_02);
    }
    
    admin_info.real_tokens_num = token_num;
    msg!("real_tokens_num is {:?}", admin_info.real_tokens_num);

    Ok(())
}

pub fn set_whitelist(
    ctx: Context<SetAdminInfo>,
    whitelist_users: [Pubkey; 10],
    user_num: u16
) -> Result<()> {
    require!(user_num <= 10, ErrorCode::UserNumTooMany);

    let admin_info = &mut ctx.accounts.admin_info;
    msg!("old whitelist is {:?}", admin_info.users);
    admin_info.users = whitelist_users;
    admin_info.real_users_num = user_num;

    msg!("new whitelist is {:?}", admin_info.users);
    msg!("real_users_num is {:?}", admin_info.real_users_num);
    Ok(())
}

pub fn set_prededuct_receivers(
    ctx: Context<SetAdminInfo>,
    prededuct_receivers: [Pubkey; 5]
) -> Result<()> {

    let admin_info = &mut ctx.accounts.admin_info;
    msg!("old prededuct_receivers is {:?}", admin_info.prededuct_receivers);
    admin_info.prededuct_receivers = prededuct_receivers;
    msg!("new prededuct_receivers is {:?}", admin_info.prededuct_receivers);
    Ok(())
}


