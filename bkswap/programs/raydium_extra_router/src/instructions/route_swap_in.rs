use amm_anchor::RouteSwapIn;
use anchor_lang::prelude::*;
use bkswap::cpi::accounts::CollectFee;
use bkswap::program::Bkswap;
use bkswap::{self};

#[derive(Accounts, Clone)]
pub struct ProxyRouteSwapIn<'info> {
    
    /// CHECK: Safe. route_swap_program_id
    pub route_swap_program_id: AccountInfo<'info>,
    /// CHECK: Safe. amm Account
    #[account(mut)]
    pub route_from_amm_program: AccountInfo<'info>,
    /// CHECK: Safe. route from amm Account
    #[account(mut)]
    pub route_from_amm: AccountInfo<'info>,
    /// CHECK: Safe. route to amm Account
    #[account(mut)]
    pub route_to_amm: AccountInfo<'info>,

    /// CHECK: Safe. Amm authority Account
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe. amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub amm_coin_vault: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub amm_pc_vault: AccountInfo<'info>,

    /// CHECK: Safe. serum dex program id
    pub serum_program: AccountInfo<'info>,
    /// CHECK: Safe. serum market Account. serum_dex program is the owner.
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// CHECK: Safe. bids Account
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    /// CHECK: Safe. asks Account
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    /// CHECK: Safe. event_q Account
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    /// CHECK: Safe. coin_vault Account
    #[account(mut)]
    pub serum_coin_vault_account: AccountInfo<'info>,
    /// CHECK: Safe. pc_vault Account
    #[account(mut)]
    pub serum_pc_vault_account: AccountInfo<'info>,
    /// CHECK: Safe. vault_signer Account
    #[account(mut)]
    pub serum_vault_signer: AccountInfo<'info>,
    /// CHECK: Safe. user source token Account. user Account to swap from.
    #[account(mut)]
    pub user_source_token_account: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub uer_route_token_account: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub user_pda_account: AccountInfo<'info>,
    /// CHECK: Safe.
    /// #[account(mut)]
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

    system_program: Program<'info, System>,
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyRouteSwapIn<'info>>
    for CpiContext<'a, 'b, 'c, 'info, RouteSwapIn<'info>>
{
    fn from(
        accounts: &mut ProxyRouteSwapIn<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, RouteSwapIn<'info>> {
        let cpi_accounts = RouteSwapIn {
            route_from_amm_program: accounts.route_from_amm_program.clone(),
            route_from_amm: accounts.route_from_amm.clone(),
            route_to_amm: accounts.route_to_amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            amm_coin_vault: accounts.amm_coin_vault.clone(),
            amm_pc_vault: accounts.amm_pc_vault.clone(),

            serum_program: accounts.serum_program.clone(),
            serum_market: accounts.serum_market.clone(),
            serum_bids: accounts.serum_bids.clone(),
            serum_asks: accounts.serum_asks.clone(),
            serum_event_queue: accounts.serum_event_queue.clone(),
            serum_coin_vault_account: accounts.serum_coin_vault_account.clone(),
            serum_pc_vault_account: accounts.serum_pc_vault_account.clone(),
            serum_vault_signer: accounts.serum_vault_signer.clone(),
            user_source_token_account: accounts.user_source_token_account.clone(),
            uer_route_token_account: accounts.uer_route_token_account.clone(),
            user_pda_account: accounts.user_pda_account.clone(),
            user_source_owner: accounts.user_source_owner.to_account_info().clone(),

            spl_token_program: accounts.spl_token_program.clone(),

        };
        let cpi_program = accounts.route_swap_program_id.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}


pub fn proxy_route_swap_in(
    ctx: Context<ProxyRouteSwapIn>,
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
    amm_anchor::route_swap_in(ctx.accounts.into(), amount_in, minimum_amount_out)
}
