use anchor_lang::prelude::*;

pub mod constant;
pub mod states;

use crate::{constant::*, states::*};

//Is the program address
declare_id!("48CqxFjUUNYc1XUDWCHx1fpJ7fssa4NWWqCzKXSM3dPA");
#[program]
pub mod blog_sol {
    use super::*; //to use the above imported libraries and files in the program, we use super to call all at once

    pub fn init_user(ctx: Context<InitUser>, name: String, avatar: String) -> Result<()> {
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

    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> Result<()> {
            // Inititalize the post and set property
        //Increment the post total and the id 
        let post_account = &mut ctx.accounts.post_account;
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        post_account.id = user_account.last_post_id;
        post_account.title = title;
        post_account.content = content;
        post_account.user = user_account.key();
        post_account.authority = authority.key();

            // increse post id by 1
        user_account.last_post_id = user_account.last_post_id
            .checked_add(1)
            .unwrap();
  // increase total count of user post by 1
        user_account.post_count = user_account.post_count
            .checked_add(1)
            .unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitUser<'info> {
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

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction()]
pub struct CreatePost<'info> {
        #[account(
            init,
            seeds = [POST_SEED, authority.key().as_ref(), &[user_account.last_post_id as u8].as_ref()],
            bump,
            payer = authority,
            space = 2376 + 8
            
        )]
    pub post_account: Account<'info, PostAccount>,

 #[account(
     mut,
     seeds = [USER_SEED, authority.key().as_ref()],
     bump,
     has_one = authority
 )]
    pub user_account: Account<'info, UserAccount>,
     
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}