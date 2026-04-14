use anchor_lang::prelude::{ program::invoke, system_instruction::transfer, * };

use crate::{ MultisigConfig, error::ErrorCode };

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    pub config: Account<'info, MultisigConfig>,

    #[account(mut, seeds = [b"vault".as_ref(), config.key().as_ref()], bump = config.vault_bump , owner = system_program::ID)]
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit_wallet(&mut self, amount: u64) -> Result<()> {
        // just a custom setting for the threshold
        require!(amount > 0, ErrorCode::InvalidAmount);

        let ix = transfer(self.depositor.key, self.vault.key, amount);
        invoke(
            &ix,
            &[
                self.depositor.to_account_info(),
                self.vault.to_account_info(),
                self.system_program.to_account_info(),
            ]
        )?;

        Ok(())
    }
}
