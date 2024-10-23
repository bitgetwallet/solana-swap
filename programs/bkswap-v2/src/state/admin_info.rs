use anchor_lang::prelude::*;

#[account]
pub struct AdminInfo {
    pub authority: Pubkey,
    pub operator: Pubkey,
    pub receiver: Pubkey,

    pub stable_token_receiver: Pubkey,
    pub other_token_receiver: Pubkey,

    pub fee_rate: u16,
    pub is_paused: bool,

    pub special_tokens_01: [Pubkey; 10],
    pub special_tokens_02: [Pubkey; 10],
    pub real_tokens_num: u16,

    pub users: [Pubkey; 10], 
    pub real_users_num: u16
}

impl AdminInfo {
    pub const LEN: usize = 32*5 + 2 + 1 + 32*20 + 2 + 32*10 + 2;
}

#[derive(Accounts)]
pub struct SetAdminInfo<'info> {
    #[account(mut,seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,
    
    #[account(address = admin_info.authority)]
    pub authority : Signer<'info>,
}
