use solana_sdk::{pubkey::Pubkey, transaction::Transaction};

pub struct TransactionBuilder {
    program_id: Pubkey,
}

impl TransactionBuilder {
    pub fn new(program_id: Pubkey) -> Self {
        TransactionBuilder { program_id }
    }

    pub fn build_swap_instruction(
        &self,
        pool: Pubkey,
        user_token_in: Pubkey,
        user_token_out: Pubkey,
        amount_in: u64,
        min_amount_out: u64,
    ) -> Result<(), String> {
        // Instruction building logic
        Ok(())
    }

    pub fn build_add_liquidity_instruction(
        &self,
        pool: Pubkey,
        user_token_a: Pubkey,
        user_token_b: Pubkey,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<(), String> {
        // Instruction building logic
        Ok(())
    }
}
