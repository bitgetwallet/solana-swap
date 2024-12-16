use anchor_lang::prelude::*;
use instructions::*;
use crate::state::*;

pub mod instructions;
pub mod errors;
pub mod state;

declare_id!("6nceGXdv8oUPgdwvJGSPzG2CgBPKNCzEtZygzuCy6P3T");

#[program]
pub mod raydium_amm_router_v3 {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>, 
        authority: Pubkey, 
        operator: Pubkey,
        receiver: Pubkey,
        bkswap_program_id: Pubkey,
        amm_program_id: Pubkey
    ) -> Result<()> {
        instructions::initialize(
            ctx,
            authority,
            operator,
            receiver,
            bkswap_program_id,
            amm_program_id
        )
    }

    /// swap_base_in instruction
    pub fn proxy_swap_base_in(
        ctx: Context<ProxySwapBaseIn>,
        amount_in: u64,
        minimum_amount_out: u64,

        prededuct_amount: u64,
        fee_rate: u16,
    ) -> Result<()> {
        let _ = instructions::proxy_swap_base_in(
            ctx,
            amount_in, 
            minimum_amount_out,
            prededuct_amount,
            fee_rate
        );

        Ok(())
    }

    /// route swap_base_in instruction
    pub fn proxy_route_swap_base_in(
        ctx: Context<ProxyRouteSwapBaseIn>,
        amount_in: u64,
        minimum_amount_out: u64,

        prededuct_amount: u64,
        fee_rate: u16,
    ) -> Result<()> {
        let _ = instructions::proxy_route_swap_base_in(
            ctx,
            amount_in, 
            minimum_amount_out,
            prededuct_amount,
            fee_rate
        );

        Ok(())
    }

    pub fn proxy_route_swap_out(
        ctx: Context<ProxyRouteSwapOut>,
        minimum_amount_out: u64
    ) -> Result<()> {
        let _ = instructions::proxy_route_swap_out(ctx, minimum_amount_out);

        Ok(())
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

    pub fn set_amm_program_id(ctx: Context<SetAdminRole>, clmm_program_id: Pubkey) -> Result<()> {
        instructions::set_amm_program_id(ctx, clmm_program_id)
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
