# Concertino 🎻

**Concertino**: A Solana AMM (Automated Market Maker) orchestrated around classical Vatican orchestral music themes.

## Overview

Concertino is a production-ready DeFi AMM on Solana featuring:
- **Smart Contract (Rust)**: Core AMM logic with constant product formula (x*y=k)
- **Backend (Rust)**: Transaction builder, pool indexing, fee management
- **Frontend (Svelte)**: Swap UI, liquidity provision, pool management

### Theme: Vatican Classical Orchestral 🎼
- **Pools** = Orchestral Sections (Strings, Woodwinds, Brass)
- **Swaps** = Musical Movements
- **Liquidity Providers** = Maestros
- **Fees** = Harmonic Rates
- **Slippage** = Tempo Variation

## Project Structure

```
concertino/
├── programs/
│   └── concertino-amm/          # Solana smart contract
│       ├── src/
│       │   ├── lib.rs
│       │   └── state.rs
│       └── Cargo.toml
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── pool.rs
│   │   └── transaction.rs
│   └── Cargo.toml
├── frontend/
│   ├── src/
│   │   ├── App.svelte
│   │   ├── components/
│   │   │   ├── SwapForm.svelte
│   │   │   └── PoolManager.svelte
│   │   └── routes/
│   ├── package.json
│   └── svelte.config.js
├── Cargo.toml
└── package.json
```

## Quick Start

### Prerequisites
- Rust 1.70+
- Node.js 18+
- Solana CLI
- Anchor framework

### Setup

```bash
# Install dependencies
cargo build
cd frontend && npm install

# Build smart contract
cd programs/concertino-amm && cargo build-sbf

# Run backend
cd backend && cargo run

# Run frontend
cd frontend && npm run dev
```

## Features (MVP)

### Smart Contract
- ✅ Initialize liquidity pools (Violin, Cello, Viola sections)
- ✅ Swap tokens with 0.3% fee
- ✅ Provide/withdraw liquidity
- ✅ Calculate price with constant product formula

### Backend
- ✅ Pool state indexing
- ✅ Transaction building
- ✅ Price calculation & slippage simulation
- ✅ Fee distribution logic

### Frontend
- ✅ Connect wallet (Phantom)
- ✅ Swap UI with price impact display
- ✅ Add/remove liquidity forms
- ✅ Real-time pool stats

## Network
**Devnet** (Solana Testnet) - ready to deploy

## License
MIT
