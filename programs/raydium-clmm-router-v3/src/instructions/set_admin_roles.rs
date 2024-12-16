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

pub fn set_bkswap_program_id(ctx: Context<SetAdminRole>, bkswap_program_id: Pubkey) -> Result<()> {
    require!(bkswap_program_id != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    let account = &mut ctx.accounts.admin_info;
    require!(bkswap_program_id.key() != account.bkswap_program_id.key(),ErrorCode::ValueCannotBeEqual);

    msg!("old bkswap_program_id is {:?}", account.bkswap_program_id);
    account.bkswap_program_id = bkswap_program_id;
    msg!("new bkswap_program_id is {:?}", account.bkswap_program_id);

    Ok(())
}

pub fn set_clmm_program_id(ctx: Context<SetAdminRole>, clmm_program_id: Pubkey) -> Result<()> {
    require!(clmm_program_id != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    let account = &mut ctx.accounts.admin_info;
    require!(clmm_program_id.key() != account.clmm_program_id.key(),ErrorCode::ValueCannotBeEqual);

    msg!("old clmm_program_id is {:?}", account.clmm_program_id);
    account.clmm_program_id = clmm_program_id;
    msg!("new clmm_program_id is {:?}", account.clmm_program_id);

    Ok(())
}

