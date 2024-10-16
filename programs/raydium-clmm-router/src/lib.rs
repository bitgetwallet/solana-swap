use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use crate::state::*;


declare_id!("2QSbPkU9N8wVesAua1sh1Aip7fM9EBUrBdNFuQDjcxYo");

#[program]
pub mod raydium_clmm_router {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>, 
        authority: Pubkey, 
        operator: Pubkey,
        receiver: Pubkey
    ) -> Result<()> {
        instructions::initialize(
            ctx,
            authority,
            operator,
            receiver,
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
        is_base_input: bool,
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
        ctx: Context<StoreOldTokenBalance>,
    ) -> Result<()> {
        instructions::store_old_balance(ctx)
    }

    pub fn proxy_swap2<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ProxySwap2<'info>>,
        // amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
    ) -> Result<()> {
        instructions::proxy_swap2(
            ctx,
            // amount,
            other_amount_threshold,
            sqrt_price_limit_x64,
            is_base_input,
        )
    }

    pub fn proxy_swap3<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ProxySwap3<'info>>,
        // amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
    ) -> Result<()> {
        instructions::proxy_swap3(
            ctx,
            // amount,
            other_amount_threshold,
            sqrt_price_limit_x64,
            is_base_input,
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

    pub fn withdraw_other_token(ctx: Context<WithdrawOtherToken>, amount: u64) -> Result<()> {
        instructions::withdraw_other_token(ctx, amount)
    }

    pub fn withdraw_lamports(ctx: Context<WithdrawLamports>) 
    -> Result<()> {
        instructions::withdraw_lamports(ctx)
    }
}
