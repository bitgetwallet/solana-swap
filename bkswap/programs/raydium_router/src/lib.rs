use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;

declare_id!("4Ji3eRdwjCg2wuuJVbRBvqkPKz2xs4tchxQ4tZNhbUfs");

#[program]
pub mod raydium_router {
    use super::*;

    /// swap_base_in instruction
    pub fn proxy_swap_base_in(
        ctx: Context<ProxySwapBaseIn>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        instructions::proxy_swap_base_in(ctx, amount_in, minimum_amount_out)
    }
}
