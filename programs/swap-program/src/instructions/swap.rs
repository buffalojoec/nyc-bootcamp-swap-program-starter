//! Instruction: SwapDia
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;

use crate::error::*;
use crate::state::*;

/// Swap assets using the DEX
pub fn swap(ctx: Context<Swap>, amount_to_swap: u64) -> Result<()> {
    // Make sure the amount is not zero
    if amount_to_swap == 0 {
        return Err(SwapProgramError::InvalidSwapZeroAmount.into());
    }

    //* Insert code */
    // Receive: The assets the user is requesting to receive in exchange:
    // (Mint, From, To)
    //* Insert code */
    // Pay: The assets the user is proposing to pay in the swap:
    // (Mint, From, To, Amount)
    //* Insert code */
    Ok(())
}

#[derive(Accounts)]
pub struct Swap<'info> {
    /// Liquidity Pool
    //* Insert code */
    /// The mint account for the asset the user is requesting to receive in
    /// exchange
    #[account(
        constraint = !receive_mint.key().eq(&pay_mint.key()) @ SwapProgramError::InvalidSwapMatchingAssets
    )]
    pub receive_mint: Box<Account<'info, token::Mint>>,
    /// The Liquidity Pool's token account for the mint of the asset the user is
    /// requesting to receive in exchange (which will be debited)
    //* Insert code */
    /// The user's token account for the mint of the asset the user is
    /// requesting to receive in exchange (which will be credited)
    //* Insert code */
    /// The mint account for the asset the user is proposing to pay in the swap
    pub pay_mint: Box<Account<'info, token::Mint>>,
    /// The Liquidity Pool's token account for the mint of the asset the user is
    /// proposing to pay in the swap (which will be credited)
    //* Insert code */
    /// The user's token account for the mint of the asset the user is
    /// proposing to pay in the swap (which will be debited)
    //* Insert code */
    /// The authority requesting to swap (user)
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Token Program: Required for transferring the assets between all token
    /// accounts involved in the swap
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
