use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_basis_points: u16,
    pub theme: String,
    pub bump: u8,
}

impl Pool {
    pub const INIT_SPACE: usize = 32 + 32 + 8 + 8 + 2 + 50 + 1;
}
