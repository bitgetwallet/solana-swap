use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
    token::{self, Token},
    token_2022::{self}
};
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts, Clone)]
pub struct WithdrawTokens<'info> {
    #[account(mut, seeds=[b"admin_info"], bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(address = admin_info.operator)]
    pub operator: Signer<'info>,

    pub mint: Option<Box<InterfaceAccount<'info, Mint>>>,

    #[account(mut, token::mint=mint)]
    pub from_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        token::mint=mint,
        token::authority=admin_info.receiver
    )]
    pub to_token_account: InterfaceAccount<'info, TokenAccount>,
    
    /// CHECK from_ata_owner
    #[account(address = from_token_account.owner)]
    pub from_ata_owner: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub token_program_2022: Option<AccountInfo<'info>>,

    /// CHECK: Optional, used for PDA withdrawals
    pub pda: Option<UncheckedAccount<'info>>,
}

#[derive(Accounts, Clone)]
pub struct WithdrawTokensPDA<'info> {
    #[account(mut, seeds=[b"admin_info"], bump)]
    pub admin_info: Box<Account<'info, AdminInfo>>,

    #[account(address = admin_info.operator)]
    pub operator: Signer<'info>,

    pub mint: Option<Box<InterfaceAccount<'info, Mint>>>,

    #[account(mut, token::mint=mint)]
    pub from_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint=mint,
        token::authority=admin_info.receiver
    )]
    pub to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    
    /// CHECK from_ata_owner
    #[account(address = from_token_account.owner)]
    pub from_ata_owner: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    pub token_program_2022: Option<AccountInfo<'info>>

}

pub fn withdraw_tokens(ctx: Context<WithdrawTokens>, amount: u64) -> Result<()> {
    require!(amount <= ctx.accounts.from_token_account.amount, ErrorCode::AmountOverBalance);
    let mint = &ctx.accounts.mint;
    let mut token_program_info = ctx.accounts.token_program.to_account_info();
    let token_program_2022 = &ctx.accounts.token_program_2022;
    let from_ata_info = ctx.accounts.from_token_account.to_account_info();
    let from_authority = ctx.accounts.from_ata_owner.to_account_info();
    let to_ata_info = ctx.accounts.to_token_account.to_account_info();

    match (mint, token_program_2022) {
        (Some(mint), Some(token_program_2022)) => {
            if from_ata_info.owner == token_program_2022.key {
                token_program_info = token_program_2022.to_account_info()
            }
            token_2022::transfer_checked(
                CpiContext::new(
                    token_program_info,
                    token_2022::TransferChecked {
                        from: from_ata_info,
                        to: to_ata_info,
                        authority: from_authority,
                        mint: mint.to_account_info(),
                    },
                ),
                amount,
                mint.decimals,
            )?;
        }
        _ => token::transfer(
            CpiContext::new(
                token_program_info,
                token::Transfer {
                    from: from_ata_info,
                    to: to_ata_info,
                    authority: from_authority,
                },
            ),
            amount,
        )?,
    };

    msg!("withdraw token is {:?}", mint.as_ref().map(|m| m.key()));
    msg!("withdraw amount is {:?}", amount);
    Ok(())
}

pub fn withdraw_tokens_pda(
    ctx: Context<WithdrawTokensPDA>,
    amount: u64,
    pda_seeds: Vec<Vec<u8>>
) -> Result<()> {
    require!(amount <= ctx.accounts.from_token_account.amount, ErrorCode::AmountOverBalance);

    let pda_seeds_refs: Vec<&[u8]> = pda_seeds.iter().map(|s| s.as_slice()).collect();
    let (derived_pda, bump) = Pubkey::find_program_address(&pda_seeds_refs, ctx.program_id);
    require!(derived_pda == *ctx.accounts.from_ata_owner.key, ErrorCode::InvalidPDA);

    let mut signing_seeds = pda_seeds;
    signing_seeds.push(vec![bump]);

    let signing_seeds_refs: Vec<&[u8]> = signing_seeds.iter().map(|s| s.as_slice()).collect();

    let mint = &ctx.accounts.mint;
    let mut token_program_info = ctx.accounts.token_program.to_account_info();
    let token_program_2022 = &ctx.accounts.token_program_2022;
    let from_ata_info = ctx.accounts.from_token_account.to_account_info();
    let to_ata_info = ctx.accounts.to_token_account.to_account_info();
    let pda = ctx.accounts.from_ata_owner.to_account_info();

    match (mint, token_program_2022) {
        (Some(mint), Some(token_program_2022)) => {
            if from_ata_info.owner == token_program_2022.key {
                token_program_info = token_program_2022.to_account_info()
            }
            token_2022::transfer_checked(
                CpiContext::new_with_signer(
                    token_program_info,
                    token_2022::TransferChecked {
                        from: from_ata_info,
                        to: to_ata_info,
                        authority: pda.to_account_info(),
                        mint: mint.to_account_info(),
                    },
                    &[&signing_seeds_refs],
                ),
                amount,
                mint.decimals,
            )?;
        }
        _ => token::transfer(
            CpiContext::new_with_signer(
                token_program_info,
                token::Transfer {
                    from: from_ata_info,
                    to: to_ata_info,
                    authority: pda.to_account_info(),
                },
                &[&signing_seeds_refs],
            ),
            amount,
        )?,
    };

    msg!("withdraw token is {:?}", mint.as_ref().map(|m| m.key()));
    msg!("withdraw amount is {:?}", amount);
    Ok(())
}