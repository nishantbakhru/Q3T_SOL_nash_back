use anchor_lang::prelude::*;

declare_id!("xigBbXdhUdZzcgDspfgPBZwWJvj6AkgQsbXGgtNDG7f");

pub mod state;
pub use state::*;

pub mod instructions;
pub use instructions::*;

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed:u64, receive:u64, deposit : u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }
}


