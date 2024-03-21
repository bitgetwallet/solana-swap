use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
mod consts;
mod state;
mod errors;

declare_id!("3Hy9MBHowHUmhmSP1KahEvvtC8e6DxzLGeFjuiBQwRUA");

#[program]
pub mod bkswap {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        authority: Pubkey,
        fee_receiver: Pubkey,
        fee_rate: u16,
    ) -> Result<()> {
        instructions::initialize(
            ctx,
            authority,
            fee_receiver,
            fee_rate,
        )
    }

    pub fn collect_fee(ctx: Context<CollectFee>, amount: u64) -> Result<u64> {
        instructions::collect_fee(ctx, amount)
    }

    pub fn set_authority(ctx: Context<SetAuthority>, authority: Pubkey) -> Result<()> {
        instructions::set_authority(ctx, authority)
    }

    pub fn set_fee_receiver(ctx: Context<SetFeeReceiver>, fee_receiver: Pubkey) -> Result<()> {
        instructions::set_fee_receiver(ctx, fee_receiver)
    }

    pub fn set_fee_rate(ctx: Context<SetFeeRate>, fee_rate: u16) -> Result<()> {
        instructions::set_fee_rate(ctx, fee_rate)
    }

    pub fn set_is_paused(ctx: Context<SetIsPaused>, is_paused: bool) -> Result<()> {
        instructions::set_is_paused(ctx, is_paused)
    }
}
