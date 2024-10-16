use anchor_lang::prelude::*;
use crate::state::*;
use crate::program::Bkswapv2;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, payer = authority, 
        space = 8 + AdminInfo::LEN,
        seeds=[b"admin_info"],
        bump
    )]
    pub admin_info: Box<Account<'info, AdminInfo>>,

    #[account(
        init, payer = authority, 
        space = 8 + FeeReceivers::LEN,
        seeds=[b"fee_receivers"],
        bump
    )]
    pub fee_receivers: Box<Account<'info, FeeReceivers>>,

    #[account(
        init, payer = authority, 
        space = 8 + FeeTokens::LEN,
        seeds=[b"fee_tokens"],
        bump
    )]
    pub fee_tokens: Box<Account<'info, FeeTokens>>,

    #[account(
        init, payer = authority, 
        space = 8 + Whitelist::LEN,
        seeds=[b"whitelist"],
        bump
    )]
    pub whitelist: Box<Account<'info, Whitelist>>,

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
    let admin_info = &mut ctx.accounts.admin_info;

    admin_info.authority = authority;
    admin_info.operator = operator;
    admin_info.receiver = receiver;
    admin_info.fee_receivers_pda = ctx.accounts.fee_receivers.key();
    admin_info.fee_rate = fee_rate;
    admin_info.fee_tokens_pda = ctx.accounts.fee_tokens.key();
    admin_info.whitelist_pda = ctx.accounts.whitelist.key();

    let fee_receivers = &mut ctx.accounts.fee_receivers;
    fee_receivers.stable_token_receiver = stable_token_receiver;
    fee_receivers.other_token_receiver = other_token_receiver;

    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.users = whitelist_users;
    whitelist.total_num = user_num;

    msg!("authority is {:?}", admin_info.authority);
    msg!("operator is {:?}", admin_info.operator);
    msg!("receiver is {:?}", admin_info.receiver);
    msg!("fee_receiver is {:?}", admin_info.fee_receivers_pda);
    msg!("fee_rate is {:?}", admin_info.fee_rate);
    // msg!("special_tokens is {:?}", fee_tokens.special_tokens);
    msg!("whitelist.users is {:?}", whitelist.users);
    Ok(())
}
