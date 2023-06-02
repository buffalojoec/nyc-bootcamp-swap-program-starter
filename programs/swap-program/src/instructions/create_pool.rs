//! Instruction: InitializePriceData
use anchor_lang::prelude::*;

use crate::state::*;

/// Initialize the program by creating the liquidity pool
pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
    // Initialize the new `LiquidityPool` state
    //* Insert code */
    Ok(())
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    /// Liquidity Pool
    //* Insert code */
    /// Rent payer
    #[account(mut)]
    pub payer: Signer<'info>,
    /// System Program: Required for creating the Liquidity Pool
    pub system_program: Program<'info, System>,
}
