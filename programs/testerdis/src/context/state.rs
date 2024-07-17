use anchor_lang::prelude::*;

#[account]
#[derive(Default)]

pub struct UserProfileVault {
    pub authority: Pubkey,
    pub name: String,
    pub age: String,
    pub status_index : u8,
    pub status_count : u8,
    pub video_index : u8,
    pub gender: String,
    pub profile_url: String,
    pub wallet_address: Pubkey,
    pub total_friend: u8,
    pub country: String,
    pub description: String,
    pub init_time: i64,
    pub withdraw_queue_header: Pubkey
}

#[account]
#[derive(InitSpace)]
pub struct QueueHeader {
    pub count: u64,   // 8
    pub seq_num: u64, // 8
    pub head: Pubkey, // 32
    pub tail: Pubkey, // 32
}
