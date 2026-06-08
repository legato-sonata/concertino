use crate::PoolInfo;
use std::collections::HashMap;

pub struct PoolManager {
    pools: HashMap<String, PoolState>,
}

pub struct PoolState {
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_basis_points: u16,
    pub theme: String,
}

impl PoolManager {
    pub fn new() -> Self {
        PoolManager {
            pools: HashMap::new(),
        }
    }

    pub fn initialize_pool(&mut self, theme: &str, fee_basis_points: u16) {
        let pool = PoolState {
            reserve_a: 1_000_000,
            reserve_b: 1_000_000,
            fee_basis_points,
            theme: theme.to_string(),
        };
        self.pools.insert(theme.to_string(), pool);
    }

    pub fn get_pool(&self, theme: &str) -> Option<PoolInfo> {
        self.pools.get(theme).map(|p| PoolInfo {
            theme: p.theme.clone(),
            reserve_a: p.reserve_a,
            reserve_b: p.reserve_b,
            fee_basis_points: p.fee_basis_points,
        })
    }

    pub fn calculate_swap(&mut self, theme: &str, amount_in: u64) -> Option<u64> {
        if let Some(pool) = self.pools.get_mut(theme) {
            // Constant product formula: (x * y = k)
            // With 0.3% fee
            let amount_in_with_fee = (amount_in as u128 * 997) / 1000;
            let numerator = amount_in_with_fee * pool.reserve_b as u128;
            let denominator = pool.reserve_a as u128 + amount_in_with_fee;
            let amount_out = (numerator / denominator) as u64;

            // Update reserves
            pool.reserve_a += amount_in;
            pool.reserve_b = pool.reserve_b.saturating_sub(amount_out);

            Some(amount_out)
        } else {
            None
        }
    }

    pub fn add_liquidity(&mut self, theme: &str, amount_a: u64, amount_b: u64) -> Option<u64> {
        if let Some(pool) = self.pools.get_mut(theme) {
            // Simplified LP calculation
            let lp_amount = ((amount_a as u128 * amount_b as u128)
                .isqrt()
                .min(((amount_a as u128 * pool.reserve_b as u128) / pool.reserve_a as u128)
                    .min(amount_b as u128))) as u64;

            pool.reserve_a += amount_a;
            pool.reserve_b += amount_b;

            Some(lp_amount)
        } else {
            None
        }
    }
}

trait IntSqrt {
    fn isqrt(self) -> Self;
}

impl IntSqrt for u128 {
    fn isqrt(self) -> Self {
        if self == 0 {
            return 0;
        }
        let mut x = self;
        let mut y = (x + 1) / 2;
        while y < x {
            x = y;
            y = (x + self / x) / 2;
        }
        x
    }
}
