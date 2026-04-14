pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AVJWJYj39UyA8PsGCRKNipjchmVJfKsFiDVoeZYGCYNq");

#[program]
pub mod anchor_multisig_wallet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
