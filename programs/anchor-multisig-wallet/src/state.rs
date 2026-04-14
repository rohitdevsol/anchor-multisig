use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MultisigConfig {
    pub creator: Pubkey, // Multisig creator
    pub bump: u8, // pda bump seed .. for config

    pub vault_bump: u8, // bump for vault
    pub threshold: u8, // minimum number of signers to transfer

    #[max_len(100)]
    pub label: String,
    #[max_len(10)]
    pub signers: Vec<Pubkey>,
}
