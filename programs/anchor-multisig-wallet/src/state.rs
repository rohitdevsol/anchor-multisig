use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MultisigConfig {
    pub creator: Pubkey, // Multisig creator
    pub bump: u8, // pda bump seed
    pub threshold: u8, // minimum number to release funds

    #[max_len(100)]
    pub label: String,
    #[max_len(10)]
    pub signers: Vec<Pubkey>,
}
