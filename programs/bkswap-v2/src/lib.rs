use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
mod consts;
mod state;
mod errors;

declare_id!("2o1ApYA73aQr6EZgdDFSAhd2c9XFPp6uKuy7B5skvbsn");

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
        //special_tokens_vec: Vec<Pubkey>,
        // special_tokens: [Pubkey; 16],
        whitelist_users: [Pubkey; 10],
        // token_num: u16,
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
        ctx: Context<SetWhitelist>,
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
        ctx: Context<SetFeeTokens>,
        special_tokens_01: [Pubkey; 10],
        special_tokens_02: [Pubkey; 10],
        token_num: u16
    ) -> Result<()> {
        instructions::set_fee_tokens(
            ctx,
            special_tokens_01,
            special_tokens_02,
            token_num
        )
    }

    pub fn collect_fee(ctx: Context<CollectFee>, amount: u64) -> Result<u64> {
        instructions::collect_fee(ctx, amount)
    }

    pub fn set_authority(ctx: Context<SetAuthority>, authority: Pubkey) -> Result<()> {
        instructions::set_authority(ctx, authority)
    }

    pub fn set_operator(ctx: Context<SetOperator>, new_operator: Pubkey) -> Result<()> {
        instructions::set_operator(ctx, new_operator)
    }

    pub fn set_receiver(ctx: Context<SetReceiver>, new_receiver: Pubkey) -> Result<()> {
        instructions::set_receiver(ctx, new_receiver)
    }

    pub fn set_stable_token_receiver(ctx: Context<SetFeeReceiver>, new_fee_receiver: Pubkey) -> Result<()> {
        instructions::set_stable_token_receiver(ctx, new_fee_receiver)
    }

    pub fn set_other_token_receiver(ctx: Context<SetFeeReceiver>, new_fee_receiver: Pubkey) -> Result<()> {
        instructions::set_other_token_receiver(ctx, new_fee_receiver)
    }

    pub fn set_fee_rate(ctx: Context<SetFeeRate>, fee_rate: u16) -> Result<()> {
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
