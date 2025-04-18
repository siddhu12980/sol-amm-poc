use anchor_lang::{prelude::*, solana_program::message};

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
};
use constant_product_curve::ConstantProduct;
use crate::state::State;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Deposit<'info>{

    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account[
        has_one=mint_a,
        has_one=mint_b,
        seeds=[b"state",seed.to_le_bytes().as_ref()],
        bump
    ]]
    pub lp_config_state: Account<'info, State>,

    
    #[account(
        seeds=[b"lp",lp_config_state.key().as_ref()],
        bump,
    )]
    pub mint_lp: Account<'info, Mint>,


    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = lp_config_state,
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        associated_token::mint = mint_b,
        associated_token::authority = lp_config_state,

    )]
    pub vault_b: Account<'info, TokenAccount>,


    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = lp_config_state,
    )]
    pub user_account_a: Account<'info, TokenAccount>,

    #[account(
        associated_token::mint = mint_b,
        associated_token::authority = lp_config_state,

    )]
    pub user_account_b: Account<'info, TokenAccount>,


    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,

    )]
    pub user_lp : Account<'info,TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,


}


impl <'info>Deposit<'info> {

    pub fn deposit(&mut self, amount:u64, max_a :u64, max_b:u64) -> Result<()>{

        assert!(amount > 0);

        let (x,y) = match self.mint_lp.supply == 0 && self.vault_a.amount ==0{
            true => (max_a,max_b),
            false => {
                let amounts = ConstantProduct::xy_deposit_amounts_from_l(self.vault_a.amount, self.vault_b.amount, self.mint_lp.supply, amount, 6).unwrap();
                (amounts.x, amounts.y)
            }
        };

        assert!(x <= max_a);

        assert!(y <= max_b);

        Ok(())
            
    }

    pub fn deposit_token(&self, is_x: bool, amount: u64) -> Result<()>{

        let (from, to ) = match  is_x {
            true => (self.user_account_a.to_account_info(), self.vault_a.to_account_info()),
            false => (self.user_account_b.to_account_info(), self.vault_b.to_account_info()),
            
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accoutns = Transfer{
            from,
            to,
            authority: self.user.to_account_info()
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accoutns);

        let transfer = transfer(cpi_context, amount);



        Ok(())
    }

    pub fn mint_lp_tokens(&self , amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo{
            mint:self.mint_lp.to_account_info(),
            to: self.user_lp.to_account_info(),
            authority: self.lp_config_state.to_account_info()
        };

        let config = b"config";
        let seed_bytes = self.lp_config_state.seed.to_le_bytes();
        let bump = [self.lp_config_state.config_bump];
        let signer_seeds: &[&[&[u8]]] = &[&[
            config,
            &seed_bytes,
            &bump,
        ]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_context, amount)?;
        Ok(())
    }
    
}




