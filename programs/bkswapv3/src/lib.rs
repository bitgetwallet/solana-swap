use anchor_lang::prelude::*;
use instructions::*;
use crate::state::*;

pub mod instructions;
mod consts;
mod state;
mod errors;

declare_id!("6ZcaZFXwhGfYqsePSMFcQMtAq7Shk83CkB1CgXcyVf1K");

#[program]
pub mod bkswapv3 {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        authority: Pubkey,
        operator: Pubkey,
        receiver: Pubkey,
        stable_token_receiver: Pubkey,
        other_token_receiver: Pubkey,
        whitelist_users: [Pubkey; 10],
        user_num: u16,

        min_fee_rate_limit: u16,
        max_fee_rate_limit: u16
    ) -> Result<()> {
        instructions::initialize(
            ctx,
            authority,
            operator,
            receiver,
            stable_token_receiver,
            other_token_receiver,
            whitelist_users,
            user_num,
            min_fee_rate_limit,
            max_fee_rate_limit
        )
    }

    pub fn set_whitelist(
        ctx: Context<SetAdminInfo>,
        whitelist_users: [Pubkey; 10],
        user_num: u16
    ) -> Result<()> {
        instructions::set_whitelist(
            ctx,
            whitelist_users,
            user_num
        )
    }

    pub fn set_fee_tokens(
        ctx: Context<SetAdminInfo>,
        special_tokens: [Pubkey; 10],
        is_tokens_01:bool,
        token_num: u16
    ) -> Result<()> {
        instructions::set_fee_tokens(
            ctx,
            special_tokens,
            is_tokens_01,
            token_num
        )
    }

    pub fn collect_fee(
        ctx: Context<CollectFee>, 
        amount: u64,
        prededuct_amount: u64,
        fee_rate: u16
    ) -> Result<u64> {
        instructions::collect_fee(ctx, amount, prededuct_amount, fee_rate)
    }

    pub fn set_authority(ctx: Context<SetAdminInfo>, authority: Pubkey) -> Result<()> {
        instructions::set_authority(ctx, authority)
    }

    pub fn set_operator(ctx: Context<SetAdminInfo>, new_operator: Pubkey) -> Result<()> {
        instructions::set_operator(ctx, new_operator)
    }

    pub fn set_receiver(ctx: Context<SetAdminInfo>, new_receiver: Pubkey) -> Result<()> {
        instructions::set_receiver(ctx, new_receiver)
    }

    pub fn set_stable_token_receiver(ctx: Context<SetAdminInfo>, new_fee_receiver: Pubkey) -> Result<()> {
        instructions::set_stable_token_receiver(ctx, new_fee_receiver)
    }

    pub fn set_other_token_receiver(ctx: Context<SetAdminInfo>, new_fee_receiver: Pubkey) -> Result<()> {
        instructions::set_other_token_receiver(ctx, new_fee_receiver)
    }

    pub fn set_prededuct_receivers(
        ctx: Context<SetAdminInfo>,
        prededuct_receivers: [Pubkey; 5]
    ) -> Result<()> {
        instructions::set_prededuct_receivers(ctx, prededuct_receivers)
    }

    pub fn set_min_fee_rate_limit(ctx: Context<SetAdminInfo>, min_fee_rate_limit: u16) -> Result<()> {
        instructions::set_min_fee_rate_limit(ctx, min_fee_rate_limit)
    }

    pub fn set_max_fee_rate_limit(ctx: Context<SetAdminInfo>, max_fee_rate_limit: u16) -> Result<()> {
        instructions::set_max_fee_rate_limit(ctx, max_fee_rate_limit)
    }

    pub fn set_is_paused(ctx: Context<SetIsPaused>, is_paused: bool) -> Result<()> {
        instructions::set_is_paused(ctx, is_paused)
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

