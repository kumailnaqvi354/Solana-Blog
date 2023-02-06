use anchor_lang::prelude::*;

#[account]
#[derive(Default)]

pub struct UserAccount {
    pub name: String,      // 4 + 256 space
    pub avatar: String,    //4 + 2048 space
    pub authority: Pubkey, //32 space
    pub last_post_id: u8,  //1 space
    pub post_count: u8,    //1 space
}

#[account]
#[derive(Default)]
pub struct PostAccount {
    pub id: u8,            //1
    pub title: String,     //4 +256
    pub content: String,   // 4 + 2048
    pub user: Pubkey,      //32
    pub authority: Pubkey, //32
}
