use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use crate::instructions::*;

declare_id!("HzNyr2z667jPZpugjo6mrE6R7FCDGrDBNxx253b4bYrj");

#[program]
pub mod amm {
    use super::*;
 
    pub fn initialize(ctx: Context<Init>,seed:u64,fee: u16, authourity: Option<Pubkey> ) -> Result<()> {
        msg!("Greetings from ,creating: {:?}", ctx.program_id);

        ctx.accounts.init(seed, fee, authourity, &ctx.bumps);
        
        Ok(())
    }


    pub fn deposit(ctx: Context<Deposit>, amount:u64, max_a:u64, max_b:u64 ) -> Result<()> {
        msg!("Greetings from ,creating: {:?}", ctx.program_id);

        ctx.accounts.deposit(amount,max_a,max_b);
        

        Ok(())
    }
}

