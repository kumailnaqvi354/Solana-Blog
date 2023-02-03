use anchor_lang::prelude::*;

pub mod constant;
pub mod states;

use crate::{constant::*, states::*};

//Is the program address
declare_id!("yf2Svhk9Atphdq4LViqguxM5A2iBXwuU7mrSfSrCs7F");
#[program]
pub mod blog_sol{
    use super::*; //to use the above imported libraries and files in the program, we use super to call all at once

    pub fn init_user(ctx:Context<InitUser>, name: String, avatar: String) -> Result<()> {
        //write logic here
       let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.name = name;
        user_account.avatar = avatar;
        user_account.last_post_id = 0;
        user_account.post_count = 0;
        user_account.authority = authority.key();

        Ok(())
            
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitUser<'info>{
    #[account(
        init,
        seeds = [USER_SEED, authority.key().as_ref()], //derives program address
        bump, //to jump to another address/avatar
        payer = authority,
        space = 2312 + 8 // calculated as per data types used in states.rs
    )]        
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program:Program <'info, System>,
}