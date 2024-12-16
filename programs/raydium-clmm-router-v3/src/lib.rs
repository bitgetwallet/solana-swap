use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use crate::state::*;


declare_id!("CpDY6k6eoDm4oLDWut2bAfMt8KLGUN92A4yRHVMo8aR3");

#[program]
pub mod raydium_clmm_router_v3 {
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

        prededuct_amount: u64,
        fee_rate: u16
    ) -> Result<()> {
        let _ = instructions::proxy_swap(
            ctx.accounts,
            ctx.remaining_accounts,
            amount,
            other_amount_threshold,
            sqrt_price_limit_x64,

            prededuct_amount,
            fee_rate
        );
        
        Ok(())
    }

    pub fn proxy_multi_swap<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ProxyMultiSwap<'info>>,
        amount: u64,
        other_amount_threshold02: u64,
        sqrt_price_limit_x64: u128,
        sqrt_price_limit_x64_02: u128,
        prededuct_amount: u64,
        fee_rate: u16,
        swap_remaining_accounts_num: u8
    ) -> Result<()> {
        instructions::proxy_multi_swap(
            ctx,
            amount,
            other_amount_threshold02,
            sqrt_price_limit_x64,
            sqrt_price_limit_x64_02,
            prededuct_amount,
            fee_rate,
            swap_remaining_accounts_num
            
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