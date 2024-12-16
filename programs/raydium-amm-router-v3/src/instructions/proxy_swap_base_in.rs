
use anchor_lang::prelude::*;
use solana_program::pubkey;

use amm_anchor::SwapBaseIn;

use bkswapv3::cpi::accounts::CollectFee;
use bkswapv3::{self};

use anchor_spl::{
    token_interface::{Mint, TokenAccount, Token2022},
    token::Token,
    associated_token::AssociatedToken,
  };

use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts, Clone)]
pub struct ProxySwapBaseIn<'info> {
    #[account(mut, seeds=[b"admin_info"], bump)]
    pub admin_info: Box<Account<'info, AdminInfo>>,

    /// CHECK: Safe
    #[account(
        address = admin_info.amm_program_id
    )]
    pub amm_program: UncheckedAccount<'info>,
    /// CHECK: Safe. amm Account
    #[account(mut)]
    pub amm: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm authority Account
    #[account()]
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: Safe. amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_coin_vault Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_pc_vault Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe.OpenBook program id
    pub market_program: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market Account. OpenBook program is the owner.
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    /// CHECK: Safe. bids Account
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    /// CHECK: Safe. asks Account
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    /// CHECK: Safe. event_q Account
    #[account(mut)]
    pub market_event_queue: UncheckedAccount<'info>,
    /// CHECK: Safe. coin_vault Account
    #[account(mut)]
    pub market_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. pc_vault Account
    #[account(mut)]
    pub market_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. vault_signer Account
    #[account(mut)]
    pub market_vault_signer: UncheckedAccount<'info>,
    /// CHECK: Safe. user source token Account. user Account to swap from.
    #[account(mut)]
    pub user_token_source: UncheckedAccount<'info>,
    /// CHECK: Safe. user destination token Account. user Account to swap to.
    #[account(mut)]
    pub user_token_destination: UncheckedAccount<'info>,
    /// CHECK: Safe. user source token Account. user Account to swap from.
    // #[account(mut)]
    // pub user_token_source: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: Safe. user destination token Account. user Account to swap to.
    #[account(mut)]
    pub user_token_destination_box: Box<InterfaceAccount<'info, TokenAccount>>,
    
    /// CHECK: Safe. user owner Account
    #[account(mut)]
    pub user_source_owner: Signer<'info>,

    /// CHECK: Safe. The spl token program
    pub token_program: Program<'info, Token>,
    

    /// CHECK: Safe
    #[account(mut)]
    pub bkswap_admin_info: UncheckedAccount<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        constraint = fee_to_token_account.mint == mint.key()
    )]
    pub fee_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut, 
        constraint = prededuct_to_token_account.mint == mint.key()
    )]
    pub prededuct_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: Safe
    #[account(
        address = admin_info.bkswap_program_id
    )]
    pub bkswap_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    
}

impl<'a, 'b, 'c, 'info> From<&mut ProxySwapBaseIn<'info>>
    for CpiContext<'a, 'b, 'c, 'info, SwapBaseIn<'info>>
{
    fn from(
        accounts: &mut ProxySwapBaseIn<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, SwapBaseIn<'info>> {
        
        // let user_token_destination_info = accounts.user_token_destination.to_account_info().clone();

        let cpi_accounts = SwapBaseIn {
            amm: accounts.amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            amm_coin_vault: accounts.amm_coin_vault.clone(),
            amm_pc_vault: accounts.amm_pc_vault.clone(),
            market_program: accounts.market_program.clone(),
            market: accounts.market.clone(),
            market_bids: accounts.market_bids.clone(),
            market_asks: accounts.market_asks.clone(),
            market_event_queue: accounts.market_event_queue.clone(),
            market_coin_vault: accounts.market_coin_vault.clone(),
            market_pc_vault: accounts.market_pc_vault.clone(),
            market_vault_signer: accounts.market_vault_signer.clone(),
            user_token_source: accounts.user_token_source.clone(),
            user_token_destination: accounts.user_token_destination.clone(),
            user_source_owner: accounts.user_source_owner.clone(),
            token_program: accounts.token_program.clone(),
        };
        let cpi_program = accounts.amm_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts, Clone)]
pub struct ProxyRouteSwapBaseIn<'info> {
    #[account(mut, seeds=[b"admin_info"], bump)]
    pub admin_info: Box<Account<'info, AdminInfo>>,

    /// CHECK: Safe
    #[account(
        address = admin_info.amm_program_id
    )]
    pub amm_program: UncheckedAccount<'info>,
    /// CHECK: Safe. amm Account
    #[account(mut)]
    pub amm: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm authority Account
    #[account()]
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: Safe. amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_coin_vault Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_pc_vault Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe.OpenBook program id
    pub market_program: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market Account. OpenBook program is the owner.
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    /// CHECK: Safe. bids Account
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    /// CHECK: Safe. asks Account
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    /// CHECK: Safe. event_q Account
    #[account(mut)]
    pub market_event_queue: UncheckedAccount<'info>,
    /// CHECK: Safe. coin_vault Account
    #[account(mut)]
    pub market_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. pc_vault Account
    #[account(mut)]
    pub market_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. vault_signer Account
    #[account(mut)]
    pub market_vault_signer: UncheckedAccount<'info>,
    /// CHECK: Safe. user source token Account. user Account to swap from.
    #[account(mut)]
    pub user_token_source: UncheckedAccount<'info>,
    /// CHECK: Safe. user destination token Account. user Account to swap to.
    #[account(mut)]
    pub user_token_destination: UncheckedAccount<'info>,
    /// CHECK: Safe. user destination token Account. user Account to swap to.
    #[account(mut)]
    pub user_token_destination_box: Box<InterfaceAccount<'info, TokenAccount>>,
    
    /// CHECK: Safe. user owner Account
    #[account(mut)]
    pub user_source_owner: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user_source_owner,
        space = 8 + AmountOut::LEN,
        seeds = [amm.key.as_ref(), user_token_destination_box.mint.as_ref(), user_source_owner.key.as_ref()],
        bump
    )]
    pub amount_out_pda: Box<Account<'info, AmountOut>>,
    
    /// CHECK: Safe. The spl token program
    pub token_program: Program<'info, Token>,


    /// CHECK: Safe
    #[account(mut)]
    pub bkswap_admin_info: UncheckedAccount<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        constraint = fee_to_token_account.mint == mint.key()
    )]
    pub fee_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut, 
        constraint = prededuct_to_token_account.mint == mint.key()
    )]
    pub prededuct_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: Safe
    #[account(
        address = admin_info.bkswap_program_id
    )]
    pub bkswap_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>
    
}
impl<'a, 'b, 'c, 'info> From<&mut ProxyRouteSwapBaseIn<'info>>
    for CpiContext<'a, 'b, 'c, 'info, SwapBaseIn<'info>>
{
    fn from(
        accounts: &mut ProxyRouteSwapBaseIn<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, SwapBaseIn<'info>> {
        
        // let user_token_destination_info = accounts.user_token_destination.to_account_info().clone();

        let cpi_accounts = SwapBaseIn {
            amm: accounts.amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            amm_coin_vault: accounts.amm_coin_vault.clone(),
            amm_pc_vault: accounts.amm_pc_vault.clone(),
            market_program: accounts.market_program.clone(),
            market: accounts.market.clone(),
            market_bids: accounts.market_bids.clone(),
            market_asks: accounts.market_asks.clone(),
            market_event_queue: accounts.market_event_queue.clone(),
            market_coin_vault: accounts.market_coin_vault.clone(),
            market_pc_vault: accounts.market_pc_vault.clone(),
            market_vault_signer: accounts.market_vault_signer.clone(),
            user_token_source: accounts.user_token_source.clone(),
            user_token_destination: accounts.user_token_destination.clone(),
            user_source_owner: accounts.user_source_owner.clone(),
            token_program: accounts.token_program.clone(),
        };
        let cpi_program = accounts.amm_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts, Clone)]
pub struct ProxyRouteSwapOut<'info> {
    #[account(mut, seeds=[b"admin_info"], bump)]
    pub admin_info: Box<Account<'info, AdminInfo>>,

    /// CHECK: Safe
    #[account(
        address = admin_info.amm_program_id
    )]
    pub amm_program: UncheckedAccount<'info>,
    /// CHECK: Safe. amm Account
    #[account(mut)]
    pub amm: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm authority Account
    #[account()]
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: Safe. amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_coin_vault Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_pc_vault Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe.OpenBook program id
    pub market_program: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market Account. OpenBook program is the owner.
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    /// CHECK: Safe. bids Account
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    /// CHECK: Safe. asks Account
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    /// CHECK: Safe. event_q Account
    #[account(mut)]
    pub market_event_queue: UncheckedAccount<'info>,
    /// CHECK: Safe. coin_vault Account
    #[account(mut)]
    pub market_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. pc_vault Account
    #[account(mut)]
    pub market_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. vault_signer Account
    #[account(mut)]
    pub market_vault_signer: UncheckedAccount<'info>,
    /// CHECK: Safe. user source token Account. user Account to swap from.
    #[account(mut)]
    pub user_token_source: UncheckedAccount<'info>,
    /// CHECK: Safe. user destination token Account. user Account to swap to.
    #[account(mut)]
    pub user_token_destination: UncheckedAccount<'info>,
    /// CHECK: Safe. user source token Account. user Account to swap from.
    #[account(
        mut, 
        constraint = user_token_source_box.mint == middle_mint.key()
    )]
    pub user_token_source_box: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: Safe. user destination token Account. user Account to swap to.
    pub user_token_destination_box: Box<InterfaceAccount<'info, TokenAccount>>,
    
    /// CHECK: Safe. user owner Account
    #[account(mut)]
    pub user_source_owner: Signer<'info>,

    #[account(
        mut,
        constraint = (amount_out_pda.user == *user_source_owner.key) && (amount_out_pda.mint == user_token_source_box.mint),
        close = user_source_owner
    )]
    pub amount_out_pda: Box<Account<'info, AmountOut>>,
    
    /// CHECK: Safe. The spl token program
    pub token_program: Program<'info, Token>,

    pub middle_mint: Box<InterfaceAccount<'info, Mint>>,// == dst_mint of swap01
    
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyRouteSwapOut<'info>>
    for CpiContext<'a, 'b, 'c, 'info, SwapBaseIn<'info>>
{
    fn from(
        accounts: &mut ProxyRouteSwapOut<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, SwapBaseIn<'info>> {
        
        let cpi_accounts = SwapBaseIn {
            amm: accounts.amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            amm_coin_vault: accounts.amm_coin_vault.clone(),
            amm_pc_vault: accounts.amm_pc_vault.clone(),
            market_program: accounts.market_program.clone(),
            market: accounts.market.clone(),
            market_bids: accounts.market_bids.clone(),
            market_asks: accounts.market_asks.clone(),
            market_event_queue: accounts.market_event_queue.clone(),
            market_coin_vault: accounts.market_coin_vault.clone(),
            market_pc_vault: accounts.market_pc_vault.clone(),
            market_vault_signer: accounts.market_vault_signer.clone(),
            user_token_source: accounts.user_token_source.clone(),
            user_token_destination: accounts.user_token_destination.clone(),
            user_source_owner: accounts.user_source_owner.clone(),
            token_program: accounts.token_program.clone(),
        };
        let cpi_program = accounts.amm_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

/// swap_base_in instruction
pub fn proxy_swap_base_in(
    ctx: Context<ProxySwapBaseIn>,
    amount_in: u64,
    minimum_amount_out: u64,

    prededuct_amount: u64,
    fee_rate: u16,
) -> Result<u64> {

    let cpi_accounts = CollectFee{
        admin_info: ctx.accounts.bkswap_admin_info.to_account_info(),
        fee_to_token_account: ctx.accounts.fee_to_token_account.to_account_info(),
        prededuct_to_token_account: ctx.accounts.prededuct_to_token_account.to_account_info(),
        user_source_token_account: ctx.accounts.user_token_source.to_account_info(),
        user_owner: ctx.accounts.user_source_owner.to_account_info(),
        
        mint: ctx.accounts.mint.to_account_info(),
        token_program_x: ctx.accounts.token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };


    let bkswap_program = ctx.accounts.bkswap_program.to_account_info();
    let cpi_ctx = CpiContext::new(bkswap_program, cpi_accounts);

    let amount_for = bkswapv3::cpi::collect_fee(
                cpi_ctx, 
                amount_in,
                prededuct_amount,
                fee_rate
            )?.get();
    
    
    let before_bal_out = ctx.accounts.user_token_destination_box.amount;
    let _ = amm_anchor::swap_base_in(ctx.accounts.into(), amount_for, minimum_amount_out);
    ctx.accounts.user_token_destination_box.reload()?;
    let after_bal_out = ctx.accounts.user_token_destination_box.amount;

    require!(after_bal_out.checked_sub(before_bal_out).ok_or(ErrorCode::ArithmeticError)? >= minimum_amount_out, ErrorCode::TooLittleOutputReceived);

    Ok(after_bal_out - before_bal_out)

}

pub fn proxy_route_swap_base_in(
    ctx: Context<ProxyRouteSwapBaseIn>,
    amount_in: u64,
    minimum_amount_out: u64,

    prededuct_amount: u64,
    fee_rate: u16,
) -> Result<u64> {

    let cpi_accounts = CollectFee{
        admin_info: ctx.accounts.bkswap_admin_info.to_account_info(),
        fee_to_token_account: ctx.accounts.fee_to_token_account.to_account_info(),
        prededuct_to_token_account: ctx.accounts.prededuct_to_token_account.to_account_info(),
        user_source_token_account: ctx.accounts.user_token_source.to_account_info(),
        user_owner: ctx.accounts.user_source_owner.to_account_info(),
        
        mint: ctx.accounts.mint.to_account_info(),
        token_program_x: ctx.accounts.token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info()
    };

    
    let bkswap_program = ctx.accounts.bkswap_program.to_account_info();
    let cpi_ctx = CpiContext::new(bkswap_program, cpi_accounts);

    let amount_for = bkswapv3::cpi::collect_fee(
                cpi_ctx, 
                amount_in,
                prededuct_amount,
                fee_rate,
            )?.get();
    
    
    let before_bal_out = ctx.accounts.user_token_destination_box.amount;
    let _ = amm_anchor::swap_base_in(ctx.accounts.into(), amount_for, minimum_amount_out);
    ctx.accounts.user_token_destination_box.reload()?;
    let after_bal_out = ctx.accounts.user_token_destination_box.amount;

    require!(after_bal_out.checked_sub(before_bal_out).ok_or(ErrorCode::ArithmeticError)? >= minimum_amount_out, ErrorCode::TooLittleOutputReceived);

    let amount_out_pda = &mut ctx.accounts.amount_out_pda;
    amount_out_pda.user = ctx.accounts.user_source_owner.key();
    amount_out_pda.mint = ctx.accounts.user_token_destination_box.mint;
    amount_out_pda.amount_out = after_bal_out - before_bal_out;

    Ok(after_bal_out - before_bal_out)

}

pub fn proxy_route_swap_out(
    ctx: Context<ProxyRouteSwapOut>,
    minimum_amount_out: u64
) -> Result<u64> {
    let amount_out_pda = &ctx.accounts.amount_out_pda;
    let amount_in02 = amount_out_pda.amount_out;

    let before_bal_out = ctx.accounts.user_token_destination_box.amount;
    let _ = amm_anchor::swap_base_in(ctx.accounts.into(), amount_in02, minimum_amount_out);
    ctx.accounts.user_token_destination_box.reload()?;
    let after_bal_out = ctx.accounts.user_token_destination_box.amount;

    require!(after_bal_out.checked_sub(before_bal_out).ok_or(ErrorCode::ArithmeticError)? >= minimum_amount_out, ErrorCode::TooLittleOutputReceived);

    Ok(after_bal_out - before_bal_out)

}


