use anchor_lang::prelude::*;

#[account]
pub struct AdminInfo {
    // v2 status
    pub authority: Pubkey,
    pub operator: Pubkey,
    pub receiver: Pubkey,

    pub stable_token_receiver: Pubkey,
    pub other_token_receiver: Pubkey,

    pub is_paused: bool,

    pub special_tokens_01: [Pubkey; 10],// stable_tokens
    pub special_tokens_02: [Pubkey; 10],
    pub real_tokens_num: u16,

    pub users: [Pubkey; 10], // whitelist users
    pub real_users_num: u16,

    // v3 additonal status
    pub prededuct_receivers: [Pubkey; 5],
    pub min_fee_rate_limit: u16,
    pub max_fee_rate_limit: u16
}

impl AdminInfo {
    pub const LEN: usize = 32*5 + 1 + 32*20 + 2 + 32*10 + 2 + 32 + 2*5 + 32*5; // 1165 + 160(null)
}

#[derive(Accounts)]
pub struct SetAdminInfo<'info> {
    #[account(mut,seeds=[b"admin_info"],bump)]
    pub admin_info: Box<Account<'info, AdminInfo>>,
    
    #[account(address = admin_info.authority)]
    pub authority : Signer<'info>,
}
