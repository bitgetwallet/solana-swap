use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use crate::state::*;


declare_id!("2WKs6hQg3cesC2D7Hxtt878beWFezbd5dJZmFmrXumnH");

#[program]
pub mod raydium_clmm_router {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>, 
        authority: Pubkey, 
        operator: Pubkey,
        receiver: Pubkey,
        bkswap_program_id: Pubkey,
        clmm_program_id: Pubkey
    ) -> Result<()> {
        instructions::initialize(
            ctx,
            authority,
            operator,
            receiver,
            bkswap_program_id,
            clmm_program_id
        )
    }

    pub fn set_is_paused(ctx: Context<SetIsPaused>, is_paused: bool) -> Result<()> {
        instructions::set_is_paused(ctx, is_paused)
    }

    pub fn proxy_swap<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ProxySwap<'info>>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool
    ) -> Result<()> {
        instructions::proxy_swap(
            ctx,
            amount,
            other_amount_threshold,
            sqrt_price_limit_x64,
            is_base_input,
        )
    }

    pub fn store_old_balance(
        ctx: Context<StoreOldTokenBalance>
    ) -> Result<()> {
        instructions::store_old_balance(ctx)
    }

    pub fn proxy_swap2<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ProxySwap2<'info>>,
        // amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool
    ) -> Result<()> {
        instructions::proxy_swap2(
            ctx,
            // amount,
            other_amount_threshold,
            sqrt_price_limit_x64,
            is_base_input
        )
    }

    pub fn proxy_swap3<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ProxySwap3<'info>>,
        // amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool
    ) -> Result<()> {
        instructions::proxy_swap3(
            ctx,
            // amount,
            other_amount_threshold,
            sqrt_price_limit_x64,
            is_base_input
        )
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

    pub fn set_clmm_program_id(ctx: Context<SetAdminRole>, clmm_program_id: Pubkey) -> Result<()> {
        instructions::set_clmm_program_id(ctx, clmm_program_id)
    }

    pub fn withdraw_tokens(ctx: Context<WithdrawTokens>, amount: u64) -> Result<()> {
        instructions::withdraw_tokens(ctx, amount)
    }

    pub fn withdraw_lamports(ctx: Context<WithdrawLamports>) 
    -> Result<()> {
        instructions::withdraw_lamports(ctx)
    }
  
    pub fn withdraw_tokens_pda(ctx: Context<WithdrawTokensPDA>, amount: u64, pda_seeds: Vec<Vec<u8>>) -> Result<()> {
      instructions::withdraw_tokens_pda(ctx, amount, pda_seeds)
    }
}
