//! Instruction: InitializePriceData
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

use crate::state::*;

/// Provide liquidity to the pool by funding it with some asset
pub fn fund_pool(ctx: Context<FundPool>, amount: u64) -> Result<()> {
    //* Insert code */
    // Deposit: (From, To, amount)
    //* Insert code */
    Ok(())
}

#[derive(Accounts)]
pub struct FundPool<'info> {
    /// Liquidity Pool
    //* Insert code */
    /// The mint account for the asset being deposited into the pool
    pub mint: Account<'info, token::Mint>,
    /// The Liquidity Pool's token account for the asset being deposited into
    /// the pool
    //* Insert code */
    /// The payer's - or Liquidity Provider's - token account for the asset
    /// being deposited into the pool
    //* Insert code */
    // Payer / Liquidity Provider
    #[account(mut)]
    pub payer: Signer<'info>,
    /// System Program: Required for creating the Liquidity Pool's token account
    /// for the asset being deposited into the pool
    pub system_program: Program<'info, System>,
    /// Token Program: Required for transferring the assets from the Liquidity
    /// Provider's token account into the Liquidity Pool's token account
    pub token_program: Program<'info, token::Token>,
    /// Associated Token Program: Required for creating the Liquidity Pool's
    /// token account for the asset being deposited into the pool
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
