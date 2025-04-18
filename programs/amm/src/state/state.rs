use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct State{

    pub seed: u64,
    pub authourity: Option<Pubkey>,
    
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,

    pub fee: u16,
    pub locked:bool,


    pub config_bump: u8,
    pub lp_bump:u8

}


