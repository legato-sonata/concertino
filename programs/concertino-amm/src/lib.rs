use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("11111111111111111111111111111111");

mod state;
use state::*;

pub trait IntSqrt {
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

#[program]
pub mod concertino_amm {
    use super::*;

    /// Initialize a liquidity pool (Orchestral Section)
    /// theme: "Violins", "Cellos", "Violas"
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        theme: String,
        fee_basis_points: u16, // 30 = 0.3%
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.token_a = ctx.accounts.token_a.key();
        pool.token_b = ctx.accounts.token_b.key();
        pool.reserve_a = 0;
        pool.reserve_b = 0;
        pool.fee_basis_points = fee_basis_points;
        pool.theme = theme;
        pool.bump = ctx.bumps.pool;

        emit!(PoolInitialized {
            token_a: pool.token_a,
            token_b: pool.token_b,
            theme: pool.theme.clone(),
            fee: fee_basis_points,
        });

        Ok(())
    }

    /// Swap tokens (Musical Movement)
    pub fn swap(ctx: Context<Swap>, amount_in: u64, min_amount_out: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        // Determine which token is input
        let (reserve_in, reserve_out, is_a_to_b) =
            if ctx.accounts.user_token_in.mint == pool.token_a {
                (pool.reserve_a, pool.reserve_b, true)
            } else {
                (pool.reserve_b, pool.reserve_a, false)
            };

        // Calculate output with fee: (amount_in * 997) / 1000 for 0.3% fee
        let amount_in_with_fee = (amount_in as u128) * 997 / 1000;
        let amount_out =
            (amount_in_with_fee * reserve_out as u128) / (reserve_in as u128 + amount_in_with_fee);

        require!(
            amount_out >= min_amount_out as u128,
            AMMError::SlippageExceeded
        );

        // Transfer tokens
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_in.to_account_info(),
            to: ctx.accounts.pool_token_in.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount_in)?;

        let cpi_accounts_out = Transfer {
            from: ctx.accounts.pool_token_out.to_account_info(),
            to: ctx.accounts.user_token_out.to_account_info(),
            authority: ctx.accounts.pool_signer.to_account_info(),
        };
        let cpi_program_out = ctx.accounts.token_program.to_account_info();
        let seeds = &[
            b"pool",
            pool.token_a.as_ref(),
            pool.token_b.as_ref(),
            &[pool.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx_out =
            CpiContext::new_with_signer(cpi_program_out, cpi_accounts_out, signer_seeds);
        token::transfer(cpi_ctx_out, amount_out as u64)?;

        // Update reserves
        if is_a_to_b {
            pool.reserve_a += amount_in;
            pool.reserve_b -= amount_out as u64;
        } else {
            pool.reserve_b += amount_in;
            pool.reserve_a -= amount_out as u64;
        }

        emit!(SwapEvent {
            amount_in,
            amount_out: amount_out as u64,
            token_a: pool.token_a,
            token_b: pool.token_b,
        });

        Ok(())
    }

    /// Add liquidity (Maestro joins the orchestra)
    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let lp_supply = ctx.accounts.lp_token_mint.supply;

        let lp_amount = if lp_supply == 0 {
            ((amount_a as u128) * (amount_b as u128))
                .isqrt()
                .try_into()
                .unwrap()
        } else {
            let amount_a_ratio = (amount_a as u128 * lp_supply as u128) / pool.reserve_a as u128;
            let amount_b_ratio = (amount_b as u128 * lp_supply as u128) / pool.reserve_b as u128;
            std::cmp::min(amount_a_ratio, amount_b_ratio) as u64
        };

        // Transfer tokens to pool
        let cpi_ctx_a = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_a.to_account_info(),
                to: ctx.accounts.pool_token_a.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(cpi_ctx_a, amount_a)?;

        let cpi_ctx_b = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_b.to_account_info(),
                to: ctx.accounts.pool_token_b.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(cpi_ctx_b, amount_b)?;

        // Mint LP tokens
        let seeds = &[
            b"pool",
            pool.token_a.as_ref(),
            pool.token_b.as_ref(),
            &[pool.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.lp_token_mint.to_account_info(),
                    to: ctx.accounts.user_lp_token.to_account_info(),
                    authority: ctx.accounts.pool_signer.to_account_info(),
                },
                signer_seeds,
            ),
            lp_amount,
        )?;

        pool.reserve_a += amount_a;
        pool.reserve_b += amount_b;

        emit!(LiquidityAdded {
            amount_a,
            amount_b,
            lp_amount,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Pool::INIT_SPACE,
        seeds = [b"pool", token_a.key().as_ref(), token_b.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    pub token_a: Account<'info, Mint>,
    pub token_b: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub user_token_in: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_out: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_in: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_out: Account<'info, TokenAccount>,
    pub pool_signer: SystemAccount<'info>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_lp_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_token_mint: Account<'info, Mint>,
    pub pool_signer: SystemAccount<'info>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct PoolInitialized {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub theme: String,
    pub fee: u16,
}

#[event]
pub struct SwapEvent {
    pub amount_in: u64,
    pub amount_out: u64,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
}

#[event]
pub struct LiquidityAdded {
    pub amount_a: u64,
    pub amount_b: u64,
    pub lp_amount: u64,
}

#[error_code]
pub enum AMMError {
    #[msg("Slippage tolerance exceeded - tempo too fast!")]
    SlippageExceeded,
}
