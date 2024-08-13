use anchor_lang::prelude::*;

declare_id!("8bgSXrP2MpwkFw3kjWZLTQh3zPBtA9dAJBDQcxC23J4j");

#[program]
pub mod vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _url: String) -> Result<()> {

        ctx.accounts.initialize(&ctx.bumps)?;

        
        Ok(())
    }

    pub fn upvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.upvote()?;
        Ok(())
    }

    pub fn downvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.downvote()?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_url:String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer : Signer<'info>,//account#1
    #[account(
        init,
        payer = payer,
        seeds = [_url.as_bytes().as_ref()],
        bump,
        space = VoteState::INIT_SPACE,
    )]
    pub vote_account: Account<'info, VoteState>,//account#2
    pub system_program: Program<'info, System>,//account#3

}

impl <'info>Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vote_account.score = 0;
        self.vote_account.bump = bumps.vote_account;


        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_url:String)]
pub struct Vote<'info>{ 
    
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [_url.as_bytes().as_ref()],
        bump,
    )]
    pub vote_account: Account<'info, VoteState>

   
}

impl <'info>Vote<'info> {
    pub fn upvote(&mut self) -> Result<()> {
        self.vote_account.score += 1;
        self.vote_account.voter = self.signer.key();
        Ok(())
    }

    pub fn downvote(&mut self) -> Result<()> {
        self.vote_account.score -= 1;
        self.vote_account.voter = self.signer.key();
        Ok(())
    }
}


#[account]
pub struct VoteState{
    pub score :i64,
    pub bump: u8,
    pub voter: Pubkey,

}

impl Space for VoteState {
    const INIT_SPACE: usize = 32+8+8+1; // 8 bytes for i64(struct) and 8 bytes for anchor discriminator( index) and 1 byte for u8
}


// when you use derive[InitSPace] it will automatically add the space for the struct but will not take into consideration the anchor discriminator
