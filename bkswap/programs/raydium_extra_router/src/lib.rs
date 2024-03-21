use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;

declare_id!("DGumNmyMhSeuNYsJLWD7VELLoENEenp99pTtDqSR9dUy");

#[program]
pub mod raydium_extra_router {
    use super::*;

    pub fn proxy_route_swap_in(
        ctx: Context<ProxyRouteSwapIn>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        instructions::proxy_route_swap_in(ctx, amount_in, minimum_amount_out)
    }

    pub fn proxy_swap_base_out(
        ctx: Context<ProxySwapBaseOut>,
        max_amount_in: u64,
        amount_out: u64,
    ) -> Result<()> {
        instructions::proxy_swap_base_out(ctx, max_amount_in, amount_out)
    }
}
