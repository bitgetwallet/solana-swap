use anchor_lang::prelude::*;

use anchor_spl::{
    token_interface::{Mint, TokenAccount, Token2022},
    token::{self, Mint as OtherMint, Token, TokenAccount as OtherTokenAccount},
  };

use raydium_amm_v3::{
    cpi,
    program::AmmV3,
    states::{AmmConfig, ObservationState, PoolState},
};

use bkswapv2::cpi::accounts::CollectFee; 
use bkswapv2::{self};
use crate::errors::ErrorCode;
use crate::state::*;


/// Memo msg for swap
pub const SWAP_MEMO_MSG: &'static [u8] = b"raydium_swap";
#[derive(Accounts)]
pub struct ProxySwap<'info> {
    #[account(mut, seeds=[b"admin_info"], bump)]
    pub admin_info: Box<Account<'info, AdminInfo>>,

    pub clmm_program: Program<'info, AmmV3>,
    /// The user performing the swap
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

    /// The user token account for output token
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

    /// CHECK: Safe
    #[account(mut)]
    pub bkswap_admin_info: UncheckedAccount<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub fee_to_token_account: UncheckedAccount<'info>,
    /// CHECK: Safe
    pub bkswap_program: AccountInfo<'info>,

    pub mint: Box<InterfaceAccount<'info, Mint>>

}

#[derive(Accounts)]
pub struct ProxySwap2<'info> {
    #[account(mut, seeds=[b"admin_info"], bump)]
    pub admin_info: Box<Account<'info, AdminInfo>>,

    pub clmm_program: Program<'info, AmmV3>,
    /// The user performing the swap
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

    /// The user token account for output token
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

    // use balance pda to store amountOut of first swap
    #[account(
        mut, 
        seeds = [b"store_old_balance".as_ref(), payer.key().as_ref()],bump, 
        constraint = old_balance_pda_account.x_mint == input_token_account.mint,
        close = payer
    )]
    old_balance_pda_account: Box<Account<'info, TokenBalance>>,
}

#[derive(Accounts)]
pub struct ProxySwap3<'info> {
    #[account(mut, seeds=[b"admin_info"], bump)]
    pub admin_info: Box<Account<'info, AdminInfo>>,

    pub clmm_program: Program<'info, AmmV3>,
    /// The user performing the swap
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

    /// The user token account for output token
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

    // use balance pda to store amountOut of first swap
    #[account(
        mut, 
        seeds = [b"store_old_balance".as_ref(), creator_old_bal_pda.key().as_ref()],bump, 
        constraint = old_balance_pda_account.x_mint == input_token_account.mint,
        close = creator_old_bal_pda
    )]
    old_balance_pda_account: Box<Account<'info, TokenBalance>>,

    /// The user pay for create old_balance_pda
    #[account(mut)]
    pub creator_old_bal_pda: Signer<'info>,// 代付者


}

#[derive(Accounts)]
pub struct StoreOldTokenBalance<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    x_mint: Account<'info, OtherMint>,
    #[account(constraint = x_token_account.mint == x_mint.key())]
    x_token_account: Account<'info, OtherTokenAccount>,

    #[account(
        init,
        payer = payer,
        space = 8 + TokenBalance::LEN,
        seeds = [b"store_old_balance".as_ref(), payer.key().as_ref()],bump,
    )]
    pub old_balance_pda_account: Box<Account<'info, TokenBalance>>,

    system_program: Program<'info, System>,
}

#[account] 
 pub struct TokenBalance {
    pub x_mint: Pubkey,
    pub token_balance: u64
 }

 impl TokenBalance {
    pub const LEN: usize = 32 + 8;
 }

 pub fn store_old_balance(ctx: Context<StoreOldTokenBalance>) -> Result<()> {
    let cur_balance = ctx.accounts.x_token_account.amount;
  
    let account = &mut ctx.accounts.old_balance_pda_account;
    if account.token_balance != cur_balance {
        account.token_balance = cur_balance;
    }
    if account.x_mint != ctx.accounts.x_mint.key() {
        account.x_mint = ctx.accounts.x_mint.key();
    }
    msg!("cur_token_balance: {:?}", cur_balance);
    Ok(())
  }


pub fn proxy_swap<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ProxySwap<'info>>,
    amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit_x64: u128,
    is_base_input: bool,
) -> Result<()> {
    require!(!ctx.accounts.admin_info.is_paused, ErrorCode::ProtocolPaused);
    // other_amount_threshold
    require!(other_amount_threshold > 0, ErrorCode::ThresholdAmountCannotBeZero);

    let cpi_accounts = CollectFee{
        admin_info: ctx.accounts.bkswap_admin_info.to_account_info(),
        user_source_token_account: ctx.accounts.input_token_account.to_account_info(),
        user_owner: ctx.accounts.payer.to_account_info(),
        fee_to_token_account: ctx.accounts.fee_to_token_account.to_account_info(),
        
        mint: Some(ctx.accounts.mint.to_account_info()),
        token_program: ctx.accounts.token_program.to_account_info(),
        token_program_2022: Some(ctx.accounts.token_program_2022.to_account_info())
    };
    let bkswap_program = ctx.accounts.bkswap_program.to_account_info();
    let cpi_ctx = CpiContext::new(bkswap_program, cpi_accounts);

    let amount_for;
    if is_base_input {
        amount_for = bkswapv2::cpi::collect_fee(cpi_ctx, amount)?.get();
    } else {
        amount_for = amount;
    }

    let cpi_accounts_swap = cpi::accounts::SwapSingleV2 {
        payer: ctx.accounts.payer.to_account_info(),
        amm_config: ctx.accounts.amm_config.to_account_info(),
        pool_state: ctx.accounts.pool_state.to_account_info(),
        input_token_account: ctx.accounts.input_token_account.to_account_info(),
        output_token_account: ctx.accounts.output_token_account.to_account_info(),
        input_vault: ctx.accounts.input_vault.to_account_info(),
        output_vault: ctx.accounts.output_vault.to_account_info(),
        observation_state: ctx.accounts.observation_state.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        token_program_2022: ctx.accounts.token_program_2022.to_account_info(),
        memo_program: ctx.accounts.memo_program.to_account_info(),
        input_vault_mint: ctx.accounts.input_vault_mint.to_account_info(),
        output_vault_mint: ctx.accounts.output_vault_mint.to_account_info(),
    };
    let cpi_context_swap = CpiContext::new(ctx.accounts.clmm_program.to_account_info(), cpi_accounts_swap)
        .with_remaining_accounts(ctx.remaining_accounts.to_vec());

    let before_bal_out = ctx.accounts.output_token_account.amount;
    let before_bal_in = ctx.accounts.input_token_account.amount;

    let _ = cpi::swap_v2(
        cpi_context_swap,
        amount_for, 
        other_amount_threshold,
        sqrt_price_limit_x64,
        is_base_input,
    );

    ctx.accounts.output_token_account.reload()?;
    let after_bal_out = ctx.accounts.output_token_account.amount;

    ctx.accounts.input_token_account.reload()?;
    let after_bal_in = ctx.accounts.input_token_account.amount;

    if is_base_input {// checkout amountOut >= minAmountOut
        require!(after_bal_out.checked_sub(before_bal_out).ok_or(ErrorCode::ArithmeticError)? >= other_amount_threshold, ErrorCode::TooLittleOutputReceived);
    } else {// check amountIn <= maxAmountIn
        require!(before_bal_in.checked_sub(after_bal_in).ok_or(ErrorCode::ArithmeticError)? <= other_amount_threshold, ErrorCode::TooMuchInputPaid);

        // collect_fee with amountOut == amount
        let cpi_accounts = CollectFee{
            admin_info: ctx.accounts.bkswap_admin_info.to_account_info(),
            user_source_token_account: ctx.accounts.output_token_account.to_account_info(),
            user_owner: ctx.accounts.payer.to_account_info(),
            fee_to_token_account: ctx.accounts.fee_to_token_account.to_account_info(),
            
            mint: Some(ctx.accounts.mint.to_account_info()),
            token_program: ctx.accounts.token_program.to_account_info(),
            token_program_2022: Some(ctx.accounts.token_program_2022.to_account_info())
        };
        let bkswap_program = ctx.accounts.bkswap_program.to_account_info();
        let cpi_ctx_fee = CpiContext::new(bkswap_program, cpi_accounts);
        let _ = bkswapv2::cpi::collect_fee(cpi_ctx_fee, amount)?.get();
    }

    Ok(())
    
}

pub fn proxy_swap2<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ProxySwap2<'info>>,
    // amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit_x64: u128,
    is_base_input: bool,
) -> Result<()> {
    require!(!ctx.accounts.admin_info.is_paused, ErrorCode::ProtocolPaused);
    // other_amount_threshold
    require!(other_amount_threshold > 0, ErrorCode::ThresholdAmountCannotBeZero);

    let new_bal_x_token = ctx.accounts.input_token_account.amount;
    let old_balance_x_token = ctx.accounts.old_balance_pda_account.token_balance;
    msg!("new_bal_x_token: {:?}", new_bal_x_token);
    msg!("old_balance_x_token: {:?}", old_balance_x_token);
    require!(new_bal_x_token > old_balance_x_token, ErrorCode::TransferAmountNeedGT0);
  
    let amount = new_bal_x_token - old_balance_x_token;

    let cpi_accounts = cpi::accounts::SwapSingleV2 {
        payer: ctx.accounts.payer.to_account_info(),
        amm_config: ctx.accounts.amm_config.to_account_info(),
        pool_state: ctx.accounts.pool_state.to_account_info(),
        input_token_account: ctx.accounts.input_token_account.to_account_info(),
        output_token_account: ctx.accounts.output_token_account.to_account_info(),
        input_vault: ctx.accounts.input_vault.to_account_info(),
        output_vault: ctx.accounts.output_vault.to_account_info(),
        observation_state: ctx.accounts.observation_state.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        token_program_2022: ctx.accounts.token_program_2022.to_account_info(),
        memo_program: ctx.accounts.memo_program.to_account_info(),
        input_vault_mint: ctx.accounts.input_vault_mint.to_account_info(),
        output_vault_mint: ctx.accounts.output_vault_mint.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.clmm_program.to_account_info(), cpi_accounts)
        .with_remaining_accounts(ctx.remaining_accounts.to_vec());

    let before_bal = ctx.accounts.output_token_account.amount;

    let _ = cpi::swap_v2(
        cpi_context,
        amount,
        other_amount_threshold,
        sqrt_price_limit_x64,
        is_base_input,
    );

    ctx.accounts.output_token_account.reload()?;
    let after_bal = ctx.accounts.output_token_account.amount;

    if is_base_input {
        require!(after_bal.checked_sub(before_bal).ok_or(ErrorCode::ArithmeticError)? >= other_amount_threshold, ErrorCode::TooLittleOutputReceived);
    }

    Ok(())
}

pub fn proxy_swap3<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ProxySwap3<'info>>,
    // amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit_x64: u128,
    is_base_input: bool,
) -> Result<()> {
    require!(!ctx.accounts.admin_info.is_paused, ErrorCode::ProtocolPaused);
    // other_amount_threshold
    require!(other_amount_threshold > 0, ErrorCode::ThresholdAmountCannotBeZero);

    let new_bal_x_token = ctx.accounts.input_token_account.amount;
    let old_balance_x_token = ctx.accounts.old_balance_pda_account.token_balance;
    msg!("new_bal_x_token: {:?}", new_bal_x_token);
    msg!("old_balance_x_token: {:?}", old_balance_x_token);
    require!(new_bal_x_token > old_balance_x_token, ErrorCode::TransferAmountNeedGT0);
  
    let amount = new_bal_x_token - old_balance_x_token;

    let cpi_accounts = cpi::accounts::SwapSingleV2 {
        payer: ctx.accounts.payer.to_account_info(),
        amm_config: ctx.accounts.amm_config.to_account_info(),
        pool_state: ctx.accounts.pool_state.to_account_info(),
        input_token_account: ctx.accounts.input_token_account.to_account_info(),
        output_token_account: ctx.accounts.output_token_account.to_account_info(),
        input_vault: ctx.accounts.input_vault.to_account_info(),
        output_vault: ctx.accounts.output_vault.to_account_info(),
        observation_state: ctx.accounts.observation_state.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        token_program_2022: ctx.accounts.token_program_2022.to_account_info(),
        memo_program: ctx.accounts.memo_program.to_account_info(),
        input_vault_mint: ctx.accounts.input_vault_mint.to_account_info(),
        output_vault_mint: ctx.accounts.output_vault_mint.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.clmm_program.to_account_info(), cpi_accounts)
        .with_remaining_accounts(ctx.remaining_accounts.to_vec());

    let before_bal = ctx.accounts.output_token_account.amount;

    let _ = cpi::swap_v2(
        cpi_context,
        amount,
        other_amount_threshold,
        sqrt_price_limit_x64,
        is_base_input,
    );

    ctx.accounts.output_token_account.reload()?;
    let after_bal = ctx.accounts.output_token_account.amount;

    if is_base_input {
        require!(after_bal.checked_sub(before_bal).ok_or(ErrorCode::ArithmeticError)? >= other_amount_threshold, ErrorCode::TooLittleOutputReceived);
    }

    Ok(())
}