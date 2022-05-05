use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod programs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
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
