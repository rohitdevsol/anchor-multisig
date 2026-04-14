pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("AVJWJYj39UyA8PsGCRKNipjchmVJfKsFiDVoeZYGCYNq");

#[program]
pub mod anchor_multisig_wallet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, threshold: u8) -> Result<()> {
        let others: Vec<Pubkey> = ctx.remaining_accounts
            .iter()
            .map(|acc| acc.key())
            .collect();
        ctx.accounts.create_wallet(threshold, others, ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_wallet(amount)
    }
}
