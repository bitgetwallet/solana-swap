use anchor_lang::prelude::*;

declare_id!("5UpKYM8aGGM5XLQHPMpnPbHR2quRyEtidC7kfYyqXEBb");

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use crate::state::*;

pub use whirlpool_cpi::state::OpenPositionBumps;

#[program]
pub mod orca_v1_clmm_router {
  use super::*;

  pub fn initialize(
    ctx: Context<Initialize>, 
    authority: Pubkey, 
    operator: Pubkey,
    receiver: Pubkey,
    bkswap_program_id: Pubkey,
    protocol_program_id: Pubkey
  ) -> Result<()> {
      instructions::initialize(
          ctx,
          authority,
          operator,
          receiver,
          bkswap_program_id,
          protocol_program_id
      )
  }

  pub fn set_is_paused(ctx: Context<SetIsPaused>, is_paused: bool) -> Result<()> {
      instructions::set_is_paused(ctx, is_paused)
  }

  pub fn set_authority(ctx: Context<SetAdminRole>, authority: Pubkey) -> Result<()> {
      instructions::set_authority(ctx, authority)
  }

  pub fn set_operator(ctx: Context<SetAdminRole>, operator: Pubkey) -> Result<()> {
      instructions::set_operator(ctx, operator)
  }

  pub fn set_receiver(ctx: Context<SetAdminRole>, receiver: Pubkey) -> Result<()> {
      instructions::set_receiver(ctx, receiver)
  }

  pub fn set_bkswap_program_id(ctx: Context<SetAdminRole>, bkswap_program_id: Pubkey) -> Result<()> {
      instructions::set_bkswap_program_id(ctx, bkswap_program_id)
  } 

  pub fn set_protocol_program_id(ctx: Context<SetAdminRole>, protocol_program_id: Pubkey) -> Result<()> {
      instructions::set_protocol_program_id(ctx, protocol_program_id)
  }

  pub fn withdraw_tokens(ctx: Context<WithdrawTokens>, amount: u64) -> Result<()> {
      instructions::withdraw_tokens(ctx, amount)
  }

  pub fn withdraw_tokens_pda(ctx: Context<WithdrawTokensPDA>, amount: u64, pda_seeds: Vec<Vec<u8>>) -> Result<()> {
      instructions::withdraw_tokens_pda(ctx, amount, pda_seeds)
  }

  pub fn withdraw_lamports(ctx: Context<WithdrawLamports>) 
  -> Result<()> {
      instructions::withdraw_lamports(ctx)
  }

  pub fn proxy_swap(
    ctx: Context<ProxySwap>,
    amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit: u128,
    amount_specified_is_input: bool,
    a_to_b: bool,
  ) -> Result<()> {
    return instructions::proxy_swap::handler_swap(
      ctx,
      amount,
      other_amount_threshold,
      sqrt_price_limit,
      amount_specified_is_input,
      a_to_b,
    );
  }

  pub fn proxy_swap_two_hop(
    ctx: Context<ProxySwapTwoHop>,
    amount: u64,
    other_amount_threshold: u64,
    amount_specified_is_input: bool,
    a_to_b_one: bool,
    a_to_b_two: bool,
    sqrt_price_limit_one: u128,
    sqrt_price_limit_two: u128   

  ) -> Result<()> {
    return instructions::proxy_swap_two_hop::handler_swap(
      ctx,
      amount,
      other_amount_threshold,
      amount_specified_is_input,
      a_to_b_one,
      a_to_b_two,
      sqrt_price_limit_one,
      sqrt_price_limit_two
    );
  }
}


