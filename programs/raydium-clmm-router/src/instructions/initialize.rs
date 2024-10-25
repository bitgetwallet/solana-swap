use anchor_lang::prelude::*;
use crate::state::*;
use crate::program::RaydiumClmmRouter;
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
    pub program: Program<'info, RaydiumClmmRouter>,
    #[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))]
    pub program_data: Account<'info, ProgramData>,
    
    system_program: Program<'info, System>,
}

pub fn initialize(
    ctx: Context<Initialize>, 
    authority: Pubkey, 
    operator: Pubkey,
    receiver: Pubkey
) -> Result<()> {
    require!(authority != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    require!(operator != Pubkey::default(), ErrorCode::AddressCannotBeNull);
    require!(receiver != Pubkey::default(), ErrorCode::AddressCannotBeNull);

    let account = &mut ctx.accounts.admin_info;
    account.authority = authority;
    account.operator = operator;
    account.receiver = receiver;

    msg!("authority is {:?}", account.authority);
    msg!("operator is {:?}", account.operator);
    msg!("receiver is {:?}", account.receiver);
    
    Ok(())
}
