use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const TEXT_LENGTH: usize = 1024;
const USER_NAME_LENGTH: usize = 100;
const USER_URL_LENGTH: usize = 255;

#[program]
pub mod programs {
    use super::*; //the scope is everything 

    pub fn create_state(
        ctx: Context<CreateState>, // everytime you create a function you put ctx and then content
        ) -> Result<()> {
        let state = &mut ctx.accounts.state; // &mut -> so we can change the authority value

        state.authority = ctx.accounts.authority.key(); // make it the current authority - user who is loged in with wallet

        state.post_count = 0;

        Ok(()) // if this functions gets called there were no errors
    }

    pub fn create_post(
        ctx: Context<CreatePost>,
        text: String,
        poster_name: String,
        poster_url: String,
    ) -> Result<()>{
        let state = &mut ctx.accounts.state;

        let post = &mut ctx.accounts.post;
        post.authority = ctx.accounts.authority.key();
        post.text = text;
        post.poster_name = poster_name;
        post.poster_url = poster_url;
        post.comment_count = 0;
        post.index = state.post_count;
        post.post_time = ctx.accounts.clock.unix_timestamp;

        state.post_count += 1; // for keeping track of post

        Ok(())
    }
}

#[derive(Account)]
pub struct CreateState<'info> {
    #[account(
        init,
        seeds = [b"state".as_ref()], //generate random seed in this state variable
        bump, //make a block unique
        payer = authority,
        space = size_of::<StateAccount>() + 8
    )]
    pub state: Account<'info StateAccount>,

    // Authority (this is signer who paid transaction fee); "mut as mutable-changable"
    #[account(mut)]
    pub authority: Signer<'info>,

    // System program
    pub system.program: UncheckedAccount<'info>, // make this part more secure in production

    // Token program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    [account(mut, seeds = [b"state".as_ref()], bump)]
    pub state: Account<'info, StateAccount>,

    // Authenticate Post Account
    #[account(
        init,

        seeds = [b"post".as_ref(), state_post_count.to_be_bytes().as_ref()], // used to create a unique seed: post account use "post" and index of post as seed
        bump,
        payer = authority,
        space = size_of::<PostAccount>() + USER_URL_LENGTH + TEXT_LENGTH + USER_NAME_LENGTH
    )]
    pub post: Account<'info, PostAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system.program: UncheckedAccount<'info>,

    #[account(constraint = token_program.key == &token::ID)] // token_program can be any token -> user needs this same token to do a transaction
    pub token_program: Program<'info, Token>,

    // Time stamp
    pub clock: Sysvar<'info, Clock>,
}



#[account]
pub struct StateAccount {
    pub authority: Pubkey,

    pub post_count: u64,
}

// Post Account Structure
#[account]
pub struct PostAccount {
    pub authority: Pubkey,

    pub text: String,

    pub poster_name: String,

    pub poster_url: String,

    pub comment_count: u64,

    pub index: u64,

    pub post_time: i64,
}
