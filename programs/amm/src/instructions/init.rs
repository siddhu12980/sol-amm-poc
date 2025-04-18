use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::*;




#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Init<'info> {
    #[account[mut]]
    pub initializer: Signer<'info>,

    pub mint_a: Account<'info, Mint>,

    pub mint_b: Account<'info, Mint>,



    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_a,
        associated_token::authority = lp_config_state,
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        init,
        payer=initializer,
        associated_token::mint = mint_b,
        associated_token::authority = lp_config_state,

    )]
    pub vault_b: Account<'info, TokenAccount>,

    #[account(
        init,
        payer=initializer,
        seeds=[b"lp",lp_config_state.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = lp_config_state,
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account[
        init,
        payer=initializer,
        seeds=[b"state",seed.to_le_bytes().as_ref()],
        space= 8 + State::INIT_SPACE,
        bump
    ]]
    pub lp_config_state: Account<'info, State>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

impl<'info> Init<'info> {
    pub fn init(&mut self, seed: u64, fee: u16, authourity: Option<Pubkey>, bumps: &InitBumps) -> Result<()> {

            self.lp_config_state.set_inner(State { seed,  authourity, mint_a: self.mint_a.key(), mint_b: self.mint_b.key(), fee, locked: false, config_bump: bumps.lp_config_state, lp_bump: bumps.mint_lp });

        Ok(())
    }


    
}
