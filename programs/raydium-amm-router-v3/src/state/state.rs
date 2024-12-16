use anchor_lang::prelude::*;

#[account]
pub struct AdminInfo {
    pub authority: Pubkey,
    pub operator: Pubkey,
    pub receiver: Pubkey,
    pub is_paused: bool,
    pub bkswap_program_id: Pubkey,
    pub amm_program_id: Pubkey,
}

impl AdminInfo {
    pub const LEN: usize = 32 * 5 + 1;
}

#[derive(Accounts)]
pub struct SetAdminRole<'info> {
    #[account(mut,seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,
    #[account(address = admin_info.authority)]
    pub authority : Signer<'info>,
}

#[account]
pub struct AmountOut {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub amount_out: u64,
}

impl AmountOut {
    pub const LEN: usize = 32 * 2 + 8;
}