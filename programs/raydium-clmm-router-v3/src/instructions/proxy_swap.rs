use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{Mint, TokenAccount, Token2022},
    token::Token,
    token_2022::ID as TOKEN_PROGRAM_2022,
    associated_token::AssociatedToken,
  };

use raydium_amm_v3::{
    cpi,
    program::AmmV3,
    states::{AmmConfig, ObservationState, PoolState},
};

use bkswapv3::cpi::accounts::CollectFee;
use bkswapv3::{self};
use crate::errors::ErrorCode;
use crate::state::*;


/// Memo msg for swap
pub const SWAP_MEMO_MSG: &'static [u8] = b"raydium_swap";
#[derive(Accounts)]
pub struct ProxySwap<'info> {
    #[account(mut, seeds=[b"admin_info"], bump)]
    pub admin_info: Box<Account<'info, AdminInfo>>,

    #[account(
        address = admin_info.clmm_program_id
    )]
    pub clmm_program: Program<'info, AmmV3>,
    /// The user performing the swap
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The factory state to read protocol fees
    #[account(address = pool_state.load()?.amm_config)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// The user token account for input token
    #[account(
        mut,
        constraint = input_token_account.mint.key() == input_vault_mint.key()
    )]
    pub input_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The user token account for output token
    #[account(
        init_if_needed, 
        payer = payer,
        associated_token::mint = output_vault_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program_y
    )]
    pub output_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for input token
    #[account(mut)]
    pub input_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for output token
    #[account(mut)]
    pub output_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The program account for the most recent oracle observation
    #[account(mut, address = pool_state.load()?.observation_key)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    /// SPL program for token transfers
    pub token_program: Program<'info, Token>,

    /// SPL program 2022 for token transfers
    pub token_program_2022: Program<'info, Token2022>,

    /// CHECK: Safe
    #[account(
        constraint = (token_program_y.key() == spl_token::ID) || (token_program_y.key() == TOKEN_PROGRAM_2022.key())
    )]
    pub token_program_y: AccountInfo<'info>,


    /// CHECK:
    #[account(
        address = spl_memo::id()
    )]
    pub memo_program: UncheckedAccount<'info>,

    /// The mint of token vault 0
    #[account(
        address = input_vault.mint
    )]
    pub input_vault_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of token vault 1
    #[account(
        address = output_vault.mint
    )]
    pub output_vault_mint: Box<InterfaceAccount<'info, Mint>>,
    // remaining accounts
    // tickarray_bitmap_extension: must add account if need regardless the sequence
    // tick_array_account_1
    // tick_array_account_2
    // tick_array_account_...

    /// CHECK: Safe
    #[account(mut)]
    pub bkswap_admin_info: UncheckedAccount<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        constraint = fee_to_token_account.mint == input_vault_mint.key()
    )]
    pub fee_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut, 
        constraint = prededuct_to_token_account.mint == input_vault_mint.key()
    )]
    pub prededuct_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: Safe
    #[account(
        address = admin_info.bkswap_program_id
    )]
    pub bkswap_program: AccountInfo<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>

}

#[derive(Accounts, Clone)]
pub struct ProxyMultiSwap<'info> {
    #[account(mut, seeds=[b"admin_info"], bump)]
    pub admin_info: Box<Account<'info, AdminInfo>>,

    #[account(
        address = admin_info.clmm_program_id
    )]
    pub clmm_program: Program<'info, AmmV3>,
    /// The user performing the swap
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The factory state to read protocol fees
    #[account(address = pool_state.load()?.amm_config)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// The user token account for input token
    #[account(mut)]
    pub input_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub output_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for input token
    #[account(mut)]
    pub input_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for output token
    #[account(mut)]
    pub output_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The program account for the most recent oracle observation
    #[account(mut, address = pool_state.load()?.observation_key)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    /// SPL program for token transfers
    pub token_program: Program<'info, Token>,

    /// SPL program 2022 for token transfers
    pub token_program_2022: Program<'info, Token2022>,

    /// CHECK:
    #[account(
        address = spl_memo::id()
    )]
    pub memo_program: UncheckedAccount<'info>,

    /// The mint of token vault 0
    #[account(
        address = input_vault.mint
    )]
    pub input_vault_mint: Box<InterfaceAccount<'info, Mint>>,

    /// The mint of token vault 1
    #[account(
        address = output_vault.mint
    )]
    pub output_vault_mint: Box<InterfaceAccount<'info, Mint>>,
    // remaining accounts
    // tickarray_bitmap_extension: must add account if need regardless the sequence
    // tick_array_account_1
    // tick_array_account_2
    // tick_array_account_...

    // pool2 accounts
    /// The factory state to read protocol fees
    #[account(address = pool_state02.load()?.amm_config)]
    pub amm_config02: Box<Account<'info, AmmConfig>>,
    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state02: AccountLoader<'info, PoolState>,

    /// The user token account for output token02
    #[account(mut)]
    pub output_token_account02: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for input token
    #[account(mut)]
    pub input_vault02: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for output token
    #[account(mut)]
    pub output_vault02: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The program account for the most recent oracle observation
    #[account(mut, address = pool_state02.load()?.observation_key)]
    pub observation_state02: AccountLoader<'info, ObservationState>,

    /// The mint of token vault 1
    #[account(
        address = output_vault02.mint
    )]
    pub output_vault_mint02: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: Safe
    #[account(mut)]
    pub bkswap_admin_info: UncheckedAccount<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        constraint = fee_to_token_account.mint == input_vault_mint.key()
    )]
    pub fee_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut, 
        constraint = prededuct_to_token_account.mint == input_vault_mint.key()
    )]
    pub prededuct_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: Safe
    #[account(
        address = admin_info.bkswap_program_id
    )]
    pub bkswap_program: AccountInfo<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

}

pub fn proxy_swap<'c: 'info, 'info>(
    ctx: &mut ProxySwap<'info>,
    remaining_accounts: &'c [AccountInfo<'info>],

    amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit_x64: u128,

    prededuct_amount: u64,
    fee_rate: u16
) -> Result<u64> {
    require!(!ctx.admin_info.is_paused, ErrorCode::ProtocolPaused);
    // other_amount_threshold
    require!(other_amount_threshold > 0, ErrorCode::ThresholdAmountCannotBeZero);

    let mut token_program_x = ctx.token_program.to_account_info();

    if token_program_x.key() != ctx.input_token_account.to_account_info().owner.key() {
        token_program_x = ctx.token_program_2022.to_account_info();
    }

    let cpi_accounts = CollectFee{
        admin_info: ctx.bkswap_admin_info.to_account_info(),
        fee_to_token_account: ctx.fee_to_token_account.to_account_info(),
        prededuct_to_token_account: ctx.prededuct_to_token_account.to_account_info(),
        user_source_token_account: ctx.input_token_account.to_account_info(),
        user_owner: ctx.payer.to_account_info(),
        
        mint: ctx.input_vault_mint.to_account_info(),
        token_program_x: token_program_x,
        system_program: ctx.system_program.to_account_info()
    };

    let bkswap_program = ctx.bkswap_program.to_account_info();
    let cpi_ctx = CpiContext::new(bkswap_program, cpi_accounts);

    let amount_for = bkswapv3::cpi::collect_fee(
                cpi_ctx, 
                amount,
                prededuct_amount,
                fee_rate
            )?.get();
    

    let cpi_accounts_swap = cpi::accounts::SwapSingleV2 {
        payer: ctx.payer.to_account_info(),
        amm_config: ctx.amm_config.to_account_info(),
        pool_state: ctx.pool_state.to_account_info(),
        input_token_account: ctx.input_token_account.to_account_info(),
        output_token_account: ctx.output_token_account.to_account_info(),
        input_vault: ctx.input_vault.to_account_info(),
        output_vault: ctx.output_vault.to_account_info(),
        observation_state: ctx.observation_state.to_account_info(),
        token_program: ctx.token_program.to_account_info(),
        token_program_2022: ctx.token_program_2022.to_account_info(),
        memo_program: ctx.memo_program.to_account_info(),
        input_vault_mint: ctx.input_vault_mint.to_account_info(),
        output_vault_mint: ctx.output_vault_mint.to_account_info(),
    };
    let cpi_context_swap = CpiContext::new(ctx.clmm_program.to_account_info(), cpi_accounts_swap)
        .with_remaining_accounts(remaining_accounts.to_vec());

    let before_bal_out = ctx.output_token_account.amount;

    let _ = cpi::swap_v2(
        cpi_context_swap,
        amount_for, 
        other_amount_threshold,
        sqrt_price_limit_x64,
        true //is_base_input,
    );

    ctx.output_token_account.reload()?;
    let after_bal_out = ctx.output_token_account.amount;

    require!(after_bal_out.checked_sub(before_bal_out).ok_or(ErrorCode::ArithmeticError)? >= other_amount_threshold, ErrorCode::TooLittleOutputReceived);

    Ok(after_bal_out - before_bal_out)
    
}

pub fn proxy_multi_swap<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ProxyMultiSwap<'info>>,
    amount: u64,
    other_amount_threshold02: u64,
    sqrt_price_limit_x64: u128,
    sqrt_price_limit_x64_02: u128,
    // is_base_input: bool
    prededuct_amount: u64,
    fee_rate: u16,
    swap_remaining_accounts_num: u8
) -> Result<()> {
    require!(!ctx.accounts.admin_info.is_paused, ErrorCode::ProtocolPaused);
    // other_amount_threshold
    require!(other_amount_threshold02 > 0, ErrorCode::ThresholdAmountCannotBeZero);


    let first_swap_remaining_accounts: &[AccountInfo<'info>] = &ctx.remaining_accounts[..swap_remaining_accounts_num as usize];
    let second_swap_remaining_accounts: &[AccountInfo<'info>] = &ctx.remaining_accounts[swap_remaining_accounts_num as usize..];

    let mut token_program_y = ctx.accounts.token_program.to_account_info();

    if token_program_y.key() != ctx.accounts.output_token_account.to_account_info().owner.key() {
        token_program_y = ctx.accounts.token_program_2022.to_account_info();
    }

    let mut swap_accounts = ProxySwap {
        admin_info: ctx.accounts.admin_info.clone(),
        clmm_program: ctx.accounts.clmm_program.clone(),
        payer: ctx.accounts.payer.clone(),
        amm_config: ctx.accounts.amm_config.clone(),
        pool_state: ctx.accounts.pool_state.clone(),
        input_token_account: ctx.accounts.input_token_account.clone(),
        output_token_account: ctx.accounts.output_token_account.clone(),
        input_vault: ctx.accounts.input_vault.clone(),
        output_vault: ctx.accounts.output_vault.clone(),
        observation_state: ctx.accounts.observation_state.clone(),
        token_program: ctx.accounts.token_program.clone(),
        token_program_2022: ctx.accounts.token_program_2022.clone(),
        token_program_y: token_program_y,
        memo_program: ctx.accounts.memo_program.clone(),
        input_vault_mint: ctx.accounts.input_vault_mint.clone(),
        output_vault_mint: ctx.accounts.output_vault_mint.clone(),
        bkswap_admin_info: ctx.accounts.bkswap_admin_info.clone(),
        fee_to_token_account: ctx.accounts.fee_to_token_account.clone(),
        prededuct_to_token_account: ctx.accounts.prededuct_to_token_account.clone(),
        bkswap_program: ctx.accounts.bkswap_program.clone(),
        associated_token_program: ctx.accounts.associated_token_program.clone(),
        system_program: ctx.accounts.system_program.clone()
    };

    let amount_out01 = proxy_swap(
        &mut swap_accounts, 
        &first_swap_remaining_accounts,
        amount, 
        1u64, 
        sqrt_price_limit_x64, 
        prededuct_amount, fee_rate
    )?;


    let cpi_accounts = cpi::accounts::SwapSingleV2 {
        payer: ctx.accounts.payer.to_account_info(),
        amm_config: ctx.accounts.amm_config02.to_account_info(),
        pool_state: ctx.accounts.pool_state02.to_account_info(),
        input_token_account: ctx.accounts.output_token_account.to_account_info(),
        output_token_account: ctx.accounts.output_token_account02.to_account_info(),
        input_vault: ctx.accounts.input_vault02.to_account_info(),
        output_vault: ctx.accounts.output_vault02.to_account_info(),
        observation_state: ctx.accounts.observation_state02.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        token_program_2022: ctx.accounts.token_program_2022.to_account_info(),
        memo_program: ctx.accounts.memo_program.to_account_info(),
        input_vault_mint: ctx.accounts.output_vault_mint.to_account_info(),
        output_vault_mint: ctx.accounts.output_vault_mint02.to_account_info(),
    };
    let cpi_context02 = CpiContext::new(ctx.accounts.clmm_program.to_account_info(), cpi_accounts)
        .with_remaining_accounts(second_swap_remaining_accounts.to_vec());

    let before_bal = ctx.accounts.output_token_account02.amount;

    let _ = cpi::swap_v2(
        cpi_context02,
        amount_out01,
        other_amount_threshold02,
        sqrt_price_limit_x64_02,
        true //is_base_input,
    );

    ctx.accounts.output_token_account02.reload()?;
    let after_bal = ctx.accounts.output_token_account02.amount;
    
    require!(after_bal.checked_sub(before_bal).ok_or(ErrorCode::ArithmeticError)? >= other_amount_threshold02, ErrorCode::TooLittleOutputReceived);

    Ok(())
}

