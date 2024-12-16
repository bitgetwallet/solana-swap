use anchor_lang::prelude::*;
use crate::state::*;
use crate::program::RaydiumAmmRouterV3;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, payer = authority, space = 8 + AdminInfo::LEN,
        seeds=[b"admin_info"],
        bump
    )]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
    pub program: Program<'info, RaydiumAmmRouterV3>,
    #[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))]
    pub program_data: Account<'info, ProgramData>,
    
    system_program: Program<'info, System>,
}

pub fn initialize(
    ctx: Context<Initialize>, 
    authority: Pubkey, 
    operator: Pubkey,
    receiver: Pubkey,
    bkswap_program_id: Pubkey,
    amm_program_id: Pubkey
) -> Result<()> {
    require!(authority != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    require!(operator != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    require!(receiver != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    require!(bkswap_program_id != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    require!(amm_program_id != Pubkey::default(), ErrorCode::AddressCannotBeNull);

    let account = &mut ctx.accounts.admin_info;
    account.authority = authority;
    account.operator = operator;
    account.receiver = receiver;
    account.bkswap_program_id = bkswap_program_id;
    account.amm_program_id = amm_program_id;

    msg!("authority is {:?}", account.authority);
    msg!("operator is {:?}", account.operator);
    msg!("receiver is {:?}", account.receiver);

    msg!("bkswap_program_id is {:?}", account.bkswap_program_id);
    msg!("amm_program_id is {:?}", account.amm_program_id);
    
    Ok(())
}
