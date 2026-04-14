use anchor_lang::prelude::*;

use crate::{ MultisigConfig, error::ErrorCode };

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + MultisigConfig::INIT_SPACE,
        seeds = [b"multisig".as_ref(), creator.key().as_ref()],
        bump
    )]
    pub config: Account<'info, MultisigConfig>,

    #[account(
        init,
        space = 0,
        payer = creator,
        seeds = [b"vault".as_ref(), config.key().as_ref()],
        bump
    )]
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn create_wallet(
        &mut self,
        threshold: u8,
        others: Vec<Pubkey>,
        bumps: InitializeBumps
    ) -> Result<()> {
        // just a custom setting for the threshold
        require!(threshold > 0 && threshold <= 10, ErrorCode::InvalidThresholdValue);

        self.config.set_inner(MultisigConfig {
            bump: bumps.config,
            vault_bump: bumps.vault,
            creator: self.creator.key(),
            label: "".to_string(),
            threshold,
            signers: others,
        });

        Ok(())
    }
}
