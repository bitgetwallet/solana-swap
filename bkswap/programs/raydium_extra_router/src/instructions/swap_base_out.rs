use amm_anchor::SwapBaseOut;
use anchor_lang::prelude::*;

#[derive(Accounts, Clone)]
pub struct ProxySwapBaseOut<'info> {

    /// CHECK: Safe. amm Account
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// CHECK: Safe. Amm authority Account
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe. amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe. amm target_orders Account
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// CHECK: Safe. pool_token_coin Amm Account to swap FROM or To,
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    /// CHECK: Safe. pool_token_pc Amm Account to swap FROM or To,
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
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
    /// CHECK: Safe. user destination token Account. user Account to swap to.
    #[account(mut)]
    pub user_destination_token_account: AccountInfo<'info>,
    /// CHECK: Safe. user owner Account
    #[account(signer)]
    pub user_source_owner: AccountInfo<'info>,
    /// CHECK: Safe. The spl token program
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,

    system_program: Program<'info, System>,
}

impl<'a, 'b, 'c, 'info> From<&mut ProxySwapBaseOut<'info>>
    for CpiContext<'a, 'b, 'c, 'info, SwapBaseOut<'info>>
{
    fn from(
        accounts: &mut ProxySwapBaseOut<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, SwapBaseOut<'info>> {
        let cpi_accounts = SwapBaseOut {
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
            user_source_owner: accounts.user_source_owner.clone(),

            spl_token_program: accounts.spl_token_program.clone(),
        };
        let cpi_program = accounts.amm.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}


pub fn proxy_swap_base_out(
    ctx: Context<ProxySwapBaseOut>,
    max_amount_in: u64,
    amount_out: u64,
) -> Result<()> {
    amm_anchor::swap_base_out(ctx.accounts.into(), max_amount_in, amount_out)
}
