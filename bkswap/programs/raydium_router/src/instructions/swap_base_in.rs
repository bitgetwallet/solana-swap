use amm_anchor::SwapBaseIn;
use anchor_lang::prelude::*;
use bkswap::cpi::accounts::CollectFee;
use bkswap::program::Bkswap;
use bkswap::{self};

#[derive(Accounts, Clone)]
pub struct ProxySwapBaseIn<'info> {
    /// CHECK: Safe
    pub amm_program: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// CHECK: Safe
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub serum_program: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub serum_coin_vault_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub serum_pc_vault_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub serum_vault_signer: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub user_source_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub user_destination_token_account: AccountInfo<'info>,
    pub user_source_owner: Signer<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub bkswap_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub fee_to_token_account: AccountInfo<'info>,
    pub bkswap_program: Program<'info, Bkswap>,
    /// CHECK: Safe
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> From<&mut ProxySwapBaseIn<'info>>
    for CpiContext<'a, 'b, 'c, 'info, SwapBaseIn<'info>>
{
    fn from(
        accounts: &mut ProxySwapBaseIn<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, SwapBaseIn<'info>> {
        let cpi_accounts = SwapBaseIn {
            amm: accounts.amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            amm_target_orders: accounts.amm_target_orders.clone(),
            pool_coin_token_account: accounts.pool_coin_token_account.clone(),
            pool_pc_token_account: accounts.pool_pc_token_account.clone(),
            serum_program: accounts.serum_program.clone(),
            serum_market: accounts.serum_market.clone(),
            serum_bids: accounts.serum_bids.clone(),
            serum_asks: accounts.serum_asks.clone(),
            serum_event_queue: accounts.serum_event_queue.clone(),
            serum_coin_vault_account: accounts.serum_coin_vault_account.clone(),
            serum_pc_vault_account: accounts.serum_pc_vault_account.clone(),
            serum_vault_signer: accounts.serum_vault_signer.clone(),
            user_source_token_account: accounts.user_source_token_account.clone(),
            user_destination_token_account: accounts.user_destination_token_account.clone(),
            user_source_owner: accounts.user_source_owner.to_account_info().clone(),
            spl_token_program: accounts.spl_token_program.clone(),
        };
        let cpi_program = accounts.amm_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

/// swap_base_in instruction
pub fn proxy_swap_base_in(
    ctx: Context<ProxySwapBaseIn>,
    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<()> {
    let cpi_accounts = CollectFee{
        bkswap_account: ctx.accounts.bkswap_account.to_account_info(),
        user_source_token_account: ctx.accounts.user_source_token_account.to_account_info(),
        user_owner: ctx.accounts.user_source_owner.to_account_info(),
        fee_to_token_account: ctx.accounts.fee_to_token_account.to_account_info(),

        spl_token_program: ctx.accounts.spl_token_program.to_account_info()
    };
    let bkswap_program = ctx.accounts.bkswap_program.to_account_info();

    let cpi_ctx = CpiContext::new(bkswap_program, cpi_accounts);
    let amount_in = bkswap::cpi::collect_fee(cpi_ctx, amount_in)?.get();
    amm_anchor::swap_base_in(ctx.accounts.into(), amount_in, minimum_amount_out)
}
