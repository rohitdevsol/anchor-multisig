use std::collections::HashSet;

use anchor_lang::prelude::{ program::{ invoke_signed }, system_instruction::transfer, * };

use crate::{ MultisigConfig, error::ErrorCode };

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        has_one = creator,
        seeds = [b"multisig".as_ref(), creator.key().as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, MultisigConfig>,

    #[account(mut, seeds = [b"vault".as_ref(), config.key().as_ref()], bump = config.vault_bump )]
    pub vault: SystemAccount<'info>,

    /// CHECK: Recipient can be any account, no data is read or trusted, only lamports are transferred
    pub recipient: UncheckedAccount<'info>, // send to this account
    pub system_program: Program<'info, System>,
}

impl<'info> Transfer<'info> {
    pub fn transfer(&self, amount: u64, ctx: &Context<Transfer>) -> Result<()> {
        // get stored signers

        require!(amount > 0, ErrorCode::InvalidAmount);

        let mut unique_signers = HashSet::new();

        for acc in ctx.remaining_accounts.iter() {
            if acc.is_signer && self.config.signers.contains(acc.key) {
                unique_signers.insert(acc.key());
            }
        }

        require!(
            unique_signers.len() >= (self.config.threshold as usize),
            ErrorCode::InvalidSigner
        );

        let binding = self.config.key();
        let seeds = &[b"vault".as_ref(), binding.as_ref(), &[self.config.vault_bump]];
        let signer_seeds = &[&seeds[..]];

        let ix = transfer(self.vault.key, self.recipient.key, amount);

        invoke_signed(
            &ix,
            &[
                self.vault.to_account_info(),
                self.recipient.to_account_info(),
                self.system_program.to_account_info(),
            ],
            signer_seeds
        )?;
        Ok(())
    }
}
