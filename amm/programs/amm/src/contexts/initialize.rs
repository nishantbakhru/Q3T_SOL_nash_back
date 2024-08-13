use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};



#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info {
    #[account(mut)]
    signer : Signer<'info>,
    mint_x: InterfaceAccount<'info,Mint>,
    mint_y: InterfaceAccount<'info,Mint>,

    #[account(
        init,
        payer = signer,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"amm", mint_x.key().as_ref(), mint_y.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    config: Account<'info, Config>,
    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"mint", config.key().as_ref()],
        bump
    )]
    
    mint_lp:
    associated_token_program: Interface<'info, TokenInterface>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info,System>,
}
