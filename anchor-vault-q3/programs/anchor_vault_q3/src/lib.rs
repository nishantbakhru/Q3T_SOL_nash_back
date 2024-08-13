use anchor_lang::{prelude::*, system_program::Transfer,system_program::transfer};

declare_id!("3kQ8hYNMGAb1gGSsYjUqYv9oL6F7ueAmUzYVcxvMpruk");

#[program]
pub mod anchor_vault_q3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        // let _ = ctx.accounts.initialize(&ctx.bumps);
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payments>, amount:u64) -> Result<()> {

        ctx.accounts.deposit(amount)?;
        
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payments>, amount: u64) -> Result<()> {

        ctx.accounts.withdraw(amount)?;
        
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {

        ctx.accounts.close()?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"state",user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub state:Account<'info,VaultState>, //solana rent 10 --url devnet
    #[account(      
        seeds = [b"vault",state.key().as_ref()],
        bump,
    )]
    pub vault:SystemAccount<'info>, //will be ata for spl token or nft
    pub system_program:Program<'info, System>,
    
}

impl <'info> Initialize<'info>{
    pub fn initialize(&mut self, bumps:&InitializeBumps) -> Result<()>{
        self.state.vault_bump = bumps.vault;
        self.state.state_bump = bumps.state;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payments<'info>{
    
    #[account(mut)]
    pub user:Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"vault",state.key().as_ref()],
        bump = state.vault_bump,
    )]
    pub vault:SystemAccount<'info>,

    #[account(
        seeds = [b"state",user.key().as_ref()],
        bump = state.state_bump,
    )]
    pub state:Account<'info,VaultState>,
    
    pub system_program:Program<'info,System>,
}

impl <'info> Payments<'info>{
    pub fn deposit(&mut self, amount:u64) -> Result<()>{

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from : self.user.to_account_info(),
            to : self.vault.to_account_info(),
        }; 

        let cpi_ctx = CpiContext::new(cpi_program,cpi_accounts);

        transfer(cpi_ctx,amount)?;

        
        Ok(())
    }

    pub fn withdraw(&mut self, amount:u64) -> Result<()>{

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from : self.vault.to_account_info(),
            to : self.user.to_account_info(),
        }; 

        let seeds = &[
            b"vault",
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);

        transfer(cpi_ctx,amount)?;

        
        Ok(())
    }
}

#[derive(Accounts)]

pub struct Close<'info>{
    #[account(mut)]
    pub user:Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault",state.key().as_ref()],
        bump = state.vault_bump,
         //close the account,
    )]
    pub vault:SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"state",user.key().as_ref()],
        bump = state.state_bump,
        close = user
    )]
    pub state:Account<'info,VaultState>,

   
    pub system_program:Program<'info,System>
}

impl <'info> Close<'info>{
    pub fn close(&mut self) -> Result<()>{

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from : self.vault.to_account_info(),
            to : self.user.to_account_info(),
        }; 

        let balance = self.vault.to_account_info().lamports();

        let seeds = &[
            b"vault",
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);

        transfer(cpi_ctx,balance)?;

        
        Ok(())
    }
    
}

#[account]

pub struct VaultState{
    pub vault_bump:u8,
    pub state_bump:u8,

}

impl Space for VaultState {    
    const INIT_SPACE: usize = 1+1+8; //2 bytes for bump and 8 bytes for u64 discriminant    
}