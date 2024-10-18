use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

pub fn set_authority(ctx: Context<SetAdminRole>, authority: Pubkey) -> Result<()> {
    require!(authority != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    let account = &mut ctx.accounts.admin_info;
    require!(authority.key() != account.authority.key(), ErrorCode::ValueCannotBeEqual);

    msg!("old authority is {:?}", account.authority);
    account.authority = authority;
    msg!("new authority is {:?}", account.authority);
    Ok(())
}

pub fn set_operator(ctx: Context<SetAdminRole>, operator: Pubkey) -> Result<()> {
    require!(operator != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    let account = &mut ctx.accounts.admin_info;
    require!(operator.key() != account.operator.key(), ErrorCode::ValueCannotBeEqual);

    msg!("old operator is {:?}", account.operator);
    account.operator = operator;
    msg!("new operator is {:?}", account.operator);
    Ok(())
}

pub fn set_receiver(ctx: Context<SetAdminRole>, receiver: Pubkey) -> Result<()> {
    require!(receiver != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    let account = &mut ctx.accounts.admin_info;
    require!(receiver.key() != account.receiver.key(), ErrorCode::ValueCannotBeEqual);

    msg!("old receiver is {:?}", account.receiver);
    account.receiver = receiver;
    msg!("new receiver is {:?}", account.receiver);

    Ok(())
}