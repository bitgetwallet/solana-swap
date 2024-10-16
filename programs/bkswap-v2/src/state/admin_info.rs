use anchor_lang::prelude::*;

#[account]
pub struct AdminInfo {
    pub authority: Pubkey,
    pub operator: Pubkey,
    pub receiver: Pubkey,

    pub fee_receivers_pda: Pubkey,
    pub fee_tokens_pda: Pubkey,// store stable token
    pub whitelist_pda: Pubkey,
    pub fee_rate: u16,
    pub is_paused: bool
}

impl AdminInfo {
    pub const LEN: usize = 32*6 + 2 + 1;
}

#[account]
pub struct FeeReceivers {
    pub stable_token_receiver: Pubkey,
    pub other_token_receiver: Pubkey,
}

impl FeeReceivers {
    pub const LEN: usize = 32*2 + 32*2;
}

#[account]
pub struct FeeTokens {
    pub special_tokens_01: [Pubkey; 10],// 16
    pub special_tokens_02: [Pubkey; 10],
    pub total_num: u16
}

impl FeeTokens {
    pub const LEN: usize = 32*30 + 2;
}

#[account]
pub struct Whitelist {
    pub users: [Pubkey; 10], 
    pub total_num: u16
}

impl Whitelist {
    pub const LEN: usize = 32*10 + 2;
}
