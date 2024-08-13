use anchor_lang::prelude::*;
pub mod state;

declare_id!("6ht7hthrdyXHx1Wq1qunUmUXkH2wvG7ovsUBFVuU3rv7");

#[program]
pub mod amm {
    use super::*;

    // Initialize a pool
    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee :u16) -> Result<()> {
        //save_config?
        
        
        Ok(())


    }
    // Add liquidity to receive LP tokens
    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y:u64) -> Result<()> {
        //deposit tokens (amount)
        //mint LP token(amount)
        Ok(())
    }
    //
    pub fn swap(ctx: Context<Swap>, amount: u64, min_receive: u64, is_x:bool, expiration : i64) -> Result<()> {
        //deposit tokens (amount)
        //withdraw tokens (amount)

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        
        //burn LP token(amount)
        //withdraw tokens (amount)
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {
    #[account(mut)]
    signer
}
