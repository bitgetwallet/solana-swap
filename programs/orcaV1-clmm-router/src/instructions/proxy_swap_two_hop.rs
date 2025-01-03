use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use anchor_spl::token_interface::Mint;
use whirlpool_cpi::{self, state::*, util::unpack::unpack_tick_array};

use bkswapv2::cpi::accounts::CollectFee; 
use bkswapv2::{self};

use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ProxySwapTwoHop<'info> {
  #[account(mut,seeds=[b"admin_info"],bump)]
  pub admin_info: Account<'info, AdminInfo>,

  #[account(
    address = admin_info.protocol_program_id
  )]
  pub whirlpool_program: Program<'info, whirlpool_cpi::program::Whirlpool>,

  #[account(address = token::ID)]
  pub token_program: Program<'info, Token>,

  pub token_authority: Signer<'info>,

  #[account(mut)]
  pub whirlpool_one: Box<Account<'info, Whirlpool>>,

  #[account(mut)]
  pub whirlpool_two: Box<Account<'info, Whirlpool>>,

  #[account(mut, constraint = token_owner_account_one_a.mint == whirlpool_one.token_mint_a)]
  pub token_owner_account_one_a: Box<Account<'info, TokenAccount>>,
  #[account(mut, address = whirlpool_one.token_vault_a)]
  pub token_vault_one_a: Box<Account<'info, TokenAccount>>,

  #[account(mut, constraint = token_owner_account_one_b.mint == whirlpool_one.token_mint_b)]
  pub token_owner_account_one_b: Box<Account<'info, TokenAccount>>,
  #[account(mut, address = whirlpool_one.token_vault_b)]
  pub token_vault_one_b: Box<Account<'info, TokenAccount>>,

  #[account(mut, constraint = token_owner_account_two_a.mint == whirlpool_two.token_mint_a)]
  pub token_owner_account_two_a: Box<Account<'info, TokenAccount>>,
  #[account(mut, address = whirlpool_two.token_vault_a)]
  pub token_vault_two_a: Box<Account<'info, TokenAccount>>,

  #[account(mut, constraint = token_owner_account_two_b.mint == whirlpool_two.token_mint_b)]
  pub token_owner_account_two_b: Box<Account<'info, TokenAccount>>,
  #[account(mut, address = whirlpool_two.token_vault_b)]
  pub token_vault_two_b: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  /// CHECK: checked in the handler
  pub tick_array_one_0: UncheckedAccount<'info>,

  #[account(mut)]
  /// CHECK: checked in the handler
  pub tick_array_one_1: UncheckedAccount<'info>,

  #[account(mut)]
  /// CHECK: checked in the handler
  pub tick_array_one_2: UncheckedAccount<'info>,

  #[account(mut)]
  /// CHECK: checked in the handler
  pub tick_array_two_0: UncheckedAccount<'info>,

  #[account(mut)]
  /// CHECK: checked in the handler
  pub tick_array_two_1: UncheckedAccount<'info>,

  #[account(mut)]
  /// CHECK: checked in the handler
  pub tick_array_two_2: UncheckedAccount<'info>,

  // #[account(seeds = [b"oracle", whirlpool_one.key().as_ref()],bump)]
  #[account(mut, seeds = [b"oracle", whirlpool_one.key().as_ref()], bump, seeds::program = whirlpool_program.key())]

  /// CHECK: Oracle is currently unused and will be enabled on subsequent updates
  pub oracle_one: UncheckedAccount<'info>,

  // #[account(seeds = [b"oracle", whirlpool_two.key().as_ref()],bump)]
  #[account(mut, seeds = [b"oracle", whirlpool_two.key().as_ref()], bump, seeds::program = whirlpool_program.key())]

  /// CHECK: Oracle is currently unused and will be enabled on subsequent updates
  pub oracle_two: UncheckedAccount<'info>,

  // --- bkswapV2.collectFee ---
  /// SPL program for token transfers
  /// CHECK: Safe
  // pub token_program: AccountInfo<'info>,

  /// SPL program 2022 for token transfers
  /// CHECK: Safe
  pub token_program_2022: AccountInfo<'info>,

  /// CHECK: Safe
  pub token_mint_a: Box<InterfaceAccount<'info, Mint>>,

  /// CHECK: Safe
  pub token_mint_b: Box<InterfaceAccount<'info, Mint>>,

  /// CHECK: Safe
  #[account(
    address = admin_info.bkswap_program_id
  )]
  pub bkswap_program: AccountInfo<'info>, 

  /// CHECK: Safe
  #[account(mut)]
  pub bkswap_admin_info: UncheckedAccount<'info>,
    
  /// CHECK: Safe
  #[account(mut)]
  pub fee_to_token_account: UncheckedAccount<'info>,
}

/*
 * params amount: amount_specified_is_input ? token_in_amout, token_out_amout
 * params other_amount_threshold: amount_specified_is_input ? token_out_min_amout, token_in_max_amout 
 * params sqrt_price_limit 
 * params amount_specified_is_input
 * params a_to_b 
*/
pub fn handler_swap(
  ctx: Context<ProxySwapTwoHop>,
  amount: u64,
  other_amount_threshold: u64,
  amount_specified_is_input: bool,
  a_to_b_one: bool,
  a_to_b_two: bool,
  sqrt_price_limit_one: u128,
  sqrt_price_limit_two: u128,    
) -> Result<()> {
  require!(!ctx.accounts.admin_info.is_paused, ErrorCode::ProtocolPaused);
  require!(other_amount_threshold > 0, ErrorCode::ThresholdAmountCannotBeZero);

  let bkswapv2_program = ctx.accounts.bkswap_program.to_account_info();
  
  let mut swap_amount = amount;
  if amount_specified_is_input {
      // exact_in: before collect_fee(token_in), after swap, check min_amount_out
      swap_amount = collect_fee(&ctx, &bkswapv2_program, amount, a_to_b_one, true)?;
      msg!("exact input: amount_in after fee: {}", swap_amount);
  } 

  let orca_cpi_program = ctx.accounts.whirlpool_program.to_account_info();
  let orca_cpi_accounts = whirlpool_cpi::cpi::accounts::TwoHopSwap {
    token_program: ctx.accounts.token_program.to_account_info(),
    token_authority: ctx.accounts.token_authority.to_account_info(),

    whirlpool_one: ctx.accounts.whirlpool_one.to_account_info(),
    token_owner_account_one_a: ctx.accounts.token_owner_account_one_a.to_account_info(),
    token_vault_one_a: ctx.accounts.token_vault_one_a.to_account_info(),
    token_owner_account_one_b: ctx.accounts.token_owner_account_one_b.to_account_info(),
    token_vault_one_b: ctx.accounts.token_vault_one_b.to_account_info(),
    tick_array_one_0: ctx.accounts.tick_array_one_0.to_account_info(),
    tick_array_one_1: ctx.accounts.tick_array_one_1.to_account_info(),
    tick_array_one_2: ctx.accounts.tick_array_one_2.to_account_info(),
    oracle_one: ctx.accounts.oracle_one.to_account_info(),
    
    whirlpool_two: ctx.accounts.whirlpool_two.to_account_info(),
    token_owner_account_two_a: ctx.accounts.token_owner_account_two_a.to_account_info(),
    token_vault_two_a: ctx.accounts.token_vault_two_a.to_account_info(),
    token_owner_account_two_b: ctx.accounts.token_owner_account_two_b.to_account_info(),
    token_vault_two_b: ctx.accounts.token_vault_two_b.to_account_info(),
    tick_array_two_0: ctx.accounts.tick_array_two_0.to_account_info(),
    tick_array_two_1: ctx.accounts.tick_array_two_1.to_account_info(),
    tick_array_two_2: ctx.accounts.tick_array_two_2.to_account_info(),
    oracle_two: ctx.accounts.oracle_two.to_account_info(),
  };

  msg!("---------------------------------------------------------------------");
  msg!("orca_cpi_program: {}", orca_cpi_program.key());
  msg!("whirlpool_one: {}", orca_cpi_accounts.whirlpool_one.key());
  msg!("whirlpool_two: {}", orca_cpi_accounts.whirlpool_two.key());
  msg!("token_program: {}", orca_cpi_accounts.token_program.key());
  msg!("token_authority: {}", orca_cpi_accounts.token_authority.key());
  msg!("token_owner_account_one_a: {}", orca_cpi_accounts.token_owner_account_one_a.key());
  msg!("token_vault_one_a: {}", orca_cpi_accounts.token_vault_one_a.key());
  msg!("token_owner_account_one_b: {}", orca_cpi_accounts.token_owner_account_one_b.key());
  msg!("token_vault_one_b: {}", orca_cpi_accounts.token_vault_one_b.key());
  msg!("tick_array_one_0: {}", orca_cpi_accounts.tick_array_one_0.key());
  msg!("tick_array_one_1: {}", orca_cpi_accounts.tick_array_one_1.key());
  msg!("tick_array_one_2: {}", orca_cpi_accounts.tick_array_one_2.key());
  msg!("oracle_one: {}", orca_cpi_accounts.oracle_one.key());
  msg!("token_owner_account_two_a: {}", orca_cpi_accounts.token_owner_account_two_a.key());
  msg!("token_vault_two_a: {}", orca_cpi_accounts.token_vault_two_a.key());
  msg!("token_owner_account_two_b: {}", orca_cpi_accounts.token_owner_account_two_b.key());
  msg!("token_vault_two_b: {}", orca_cpi_accounts.token_vault_two_b.key());
  msg!("tick_array_two_0: {}", orca_cpi_accounts.tick_array_two_0.key());
  msg!("tick_array_two_1: {}", orca_cpi_accounts.tick_array_two_1.key());
  msg!("tick_array_two_2: {}", orca_cpi_accounts.tick_array_two_2.key());
  msg!("oracle_two: {}", orca_cpi_accounts.oracle_two.key());
  msg!("token_mint_a: {}", ctx.accounts.token_mint_a.to_account_info().key());
  msg!("token_mint_b: {}", ctx.accounts.token_mint_b.to_account_info().key());
  
  msg!("whirlpool orca_swap: params");
  msg!("amount: {}", swap_amount);
  msg!("other_amount_threshold: {}", other_amount_threshold);
  msg!("amount_specified_is_input: {}", amount_specified_is_input);
  msg!("a_to_b_one: {}", a_to_b_one);
  msg!("a_to_b_two: {}", a_to_b_two);
  msg!("sqrt_price_limit_one: {}", sqrt_price_limit_one);
  msg!("sqrt_price_limit_two: {}", sqrt_price_limit_two);

  let (token_in_before_balance, _) = get_token_balances_one(&ctx, a_to_b_one);
  let (_, token_out_before_balance) = get_token_balances_two(&ctx, a_to_b_two);
  msg!("token_in_before_balance: {}", token_in_before_balance);
  msg!("token_out_before_balance: {}", token_out_before_balance);

  let orca_cpi_ctx = CpiContext::new(orca_cpi_program, orca_cpi_accounts);
  whirlpool_cpi::cpi::two_hop_swap(
    orca_cpi_ctx,
    swap_amount,
    other_amount_threshold,
    amount_specified_is_input,
    a_to_b_one,
    a_to_b_two,
    sqrt_price_limit_one,
    sqrt_price_limit_two
  )?;

  ctx.accounts.token_owner_account_one_a.reload()?;
  ctx.accounts.token_owner_account_one_b.reload()?;
  ctx.accounts.token_owner_account_two_a.reload()?;
  ctx.accounts.token_owner_account_two_b.reload()?;

  let (token_in_after_balance, _) = get_token_balances_one(&ctx, a_to_b_one);
  let (_, token_out_after_balance) = get_token_balances_two(&ctx, a_to_b_two);

  msg!("token_in_after_balance: {}", token_in_after_balance);
  msg!("token_out_after_balance: {}", token_out_after_balance);

  if amount_specified_is_input {
      require!(
        token_out_after_balance
            .checked_sub(token_out_before_balance)
            .ok_or(ErrorCode::ArithmeticError)? >= other_amount_threshold,
        ErrorCode::TooLittleOutputReceived
    );
  } else {
    // exact_out: before swap, after collect_fee(token_out), check max_amount_in
    let amount_out_after_fee = collect_fee(&ctx, &bkswapv2_program, swap_amount, a_to_b_two, false)?;
    msg!("exact output amount after_out fee: {}", amount_out_after_fee);
    require!(
      token_in_before_balance
          .checked_sub(token_in_after_balance)
          .ok_or(ErrorCode::ArithmeticError)? <= other_amount_threshold,
      ErrorCode::TooMuchInputPaid
    );
  }
  Ok(())
}

fn get_token_balances_one(ctx: &Context<ProxySwapTwoHop>, a_to_b: bool) -> (u64, u64) {
  if a_to_b {
      (ctx.accounts.token_owner_account_one_a.amount, ctx.accounts.token_owner_account_one_b.amount)
  } else {
      (ctx.accounts.token_owner_account_one_b.amount, ctx.accounts.token_owner_account_one_a.amount)
  }
}

fn get_token_balances_two(ctx: &Context<ProxySwapTwoHop>, a_to_b: bool) -> (u64, u64) {
  if a_to_b {
      (ctx.accounts.token_owner_account_two_a.amount, ctx.accounts.token_owner_account_two_b.amount)
  } else {
      (ctx.accounts.token_owner_account_two_b.amount, ctx.accounts.token_owner_account_two_a.amount)
  }
}

fn collect_fee<'info>(
  ctx: &Context<ProxySwapTwoHop<'info>>,
  bkswap_program: &AccountInfo<'info>,
  amount: u64,
  a_to_b: bool,
  is_first_collect_fee: bool
) -> Result<u64> {
  msg!("bkswapv2 collect_fee");
  
  // is_first_collect_fee? token_a : token_b
  let user_pay_fee_token_account;
  let user_pay_fee_token_mint;
  if is_first_collect_fee {
      if a_to_b {
        user_pay_fee_token_account = &ctx.accounts.token_owner_account_one_a
      } else {
        user_pay_fee_token_account = &ctx.accounts.token_owner_account_one_b
      };
      user_pay_fee_token_mint = &ctx.accounts.token_mint_a;
  } else {
      if a_to_b {
        user_pay_fee_token_account = &ctx.accounts.token_owner_account_two_b
      } else {
        user_pay_fee_token_account = &ctx.accounts.token_owner_account_two_a
      };
      user_pay_fee_token_mint = &ctx.accounts.token_mint_b;
  }

  let bkswapv2_cpi_accounts = CollectFee {
      admin_info: ctx.accounts.bkswap_admin_info.to_account_info(),
      user_owner: ctx.accounts.token_authority.to_account_info(),
      fee_to_token_account: ctx.accounts.fee_to_token_account.to_account_info(),
     
      user_source_token_account: user_pay_fee_token_account.to_account_info(),
      mint: Some(user_pay_fee_token_mint.to_account_info()),
      
      token_program: ctx.accounts.token_program.to_account_info(),
      token_program_2022: Some(ctx.accounts.token_program_2022.to_account_info())
  };
  
  msg!("bkswap_program: {}", bkswap_program.key());
  msg!("admin_info_pda: {}", bkswapv2_cpi_accounts.admin_info.key());
  msg!("user_source_token_account: {}", bkswapv2_cpi_accounts.user_source_token_account.key());
  msg!("user_owner: {}", bkswapv2_cpi_accounts.user_owner.key());
  msg!("fee_to_token_account: {}", bkswapv2_cpi_accounts.fee_to_token_account.key());
  msg!("mint: {:?}", bkswapv2_cpi_accounts.mint.as_ref().map(|acc| acc.key()));
  msg!("token_program: {}", bkswapv2_cpi_accounts.token_program.key());
  msg!("token_program_2022: {:?}", bkswapv2_cpi_accounts.token_program_2022.as_ref().map(|acc| acc.key()));

  msg!("bkswapv2 collect_fee: before amount: {}", amount);
  let bkswapv2_cpi_ctx = CpiContext::new(bkswap_program.clone(), bkswapv2_cpi_accounts);
  let amount_after_fee = bkswapv2::cpi::collect_fee(bkswapv2_cpi_ctx, amount)?.get();
  msg!("bkswapv2 collect_fee: after: amount: {}", amount_after_fee);

  Ok(amount_after_fee)
}