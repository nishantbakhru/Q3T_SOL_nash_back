use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct Config{
    seed: u64,
    fees: u16, //65536 fits 100%
    mint_x: Pubkey,
    mint_y: Pubkey,
    bump: u8,
}