use anchor_lang::prelude::*;
use instructions::*;
use crate::state::*;

pub mod instructions;
mod consts;
mod state;
mod errors;

declare_id!("Fv3fcy7DJRNdQu5618oJ8FGe7LdQ7oWGUj5oVVxFweTv");

#[program]
pub mod bkswapv2 {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        authority: Pubkey,
        operator: Pubkey,
        receiver: Pubkey,
        stable_token_receiver: Pubkey,
        other_token_receiver: Pubkey,
        fee_rate: u16,
        whitelist_users: [Pubkey; 10],
        user_num: u16
    ) -> Result<()> {
        instructions::initialize(
            ctx,
            authority,
            operator,
            receiver,
            stable_token_receiver,
            other_token_receiver,
            fee_rate,
            whitelist_users,
            user_num,

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

    pub fn collect_fee(ctx: Context<CollectFee>, amount: u64) -> Result<u64> {
        instructions::collect_fee(ctx, amount)
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

    pub fn set_fee_rate(ctx: Context<SetAdminInfo>, fee_rate: u16) -> Result<()> {
        instructions::set_fee_rate(ctx, fee_rate)
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
