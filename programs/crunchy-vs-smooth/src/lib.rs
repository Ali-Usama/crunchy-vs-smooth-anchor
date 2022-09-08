extern crate core;

use anchor_lang::prelude::*;

declare_id!("4gaMHhonL3tzdLsiKWyB1v9w6DfJ4ntrBnfP6xzm7zoQ");

#[derive(Accounts)]
pub struct Initialize<'info> {
    // init attribute creates a new account owned by the program
    // system program is required by the runtime
    #[account(init, payer=user, space = 16+16)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    // Marking accounts as mut persists any changes made upon exiting the program, allowing our votes to be recorded.
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
}

// The VoteAccount will be parsed inside each Transaction Instruction to record votes as they occur
#[account]
pub struct VoteAccount {
    pub crunchy: u64,
    pub smooth: u64,
}


#[program]
pub mod crunchy_vs_smooth {
    use super::*;

    /// Each method in here defines an RPC request handler (aka instruction handler) which can be invoked by clients
    /// The first parameter for every RPC handler is the Context struct.

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.crunchy = 0;
        vote_account.smooth = 0;
        Ok(())
    }

    /// All account validation logic is handled below at the #[account()] macros, letting us just focus on the business logic

    pub fn vote_crunchy(ctx: Context<Vote>) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.crunchy += 1;
        Ok(())
    }

    pub fn vote_smooth(ctx: Context<Vote>) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.smooth += 1;
        Ok(())
    }
}

