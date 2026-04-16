use anchor_lang::prelude::{ program::invoke, system_instruction::transfer, * };
use crate::{ MultisigConfig, error::ErrorCode };

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,  // FIX: Add signer to validate depositor

    #[account(seeds = [b"multisig".as_ref(), config.creator.key().as_ref()], bump = config.bump)]
    pub config: Account<'info, MultisigConfig>,

    #[account(mut, seeds = [b"vault".as_ref(), config.key().as_ref()], bump = config.vault_bump )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit_wallet(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        // FIX: Validate depositor is either creator or in signers list
        let is_creator = self.depositor.key() == self.config.creator;
        let is_signer = self.config.signers.contains(&self.depositor.key());
        require!(is_creator || is_signer, ErrorCode::Unauthorized);

        let ix = transfer(self.depositor.key, self.vault.key, amount);
        invoke(
            &ix,
            &[
                self.depositor.to_account_info(),
                self.vault.to_account_info(),
                self.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}
