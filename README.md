# DEX Pinocchio CPI

[中文文档](./README_CN.md)

Pinocchio-compatible CPI client library for Solana DEX programs. Supports 35 major DEX protocols.

## Features

- **Pure Pinocchio**: Ultra-lightweight implementation
- **no_std Compatible**: Works in resource-constrained environments
- **35 DEX Protocols**: Covers major DEXes in the Solana ecosystem
- **Type-safe**: All instruction arguments have strict type definitions

## Project Structure

```
dex-pinocchio-cpi/
├── Cargo.toml              # Project configuration
├── README.md               # This document
├── README_CN.md            # Chinese documentation
├── dex_idls/               # IDL source files (34 DEX IDLs)
│   ├── Pump_fun.json
│   ├── Raydium_CLMM.json
│   ├── Meteora_DLMM.json
│   └── ...
└── src/
    ├── lib.rs              # Library entry, exports all modules
    ├── pump_fun.rs         # Pump.fun CPI module
    ├── raydium_clmm.rs     # Raydium CLMM CPI module
    ├── meteora_dlmm.rs     # Meteora DLMM CPI module
    └── ...                 # 34 DEX modules total
```

## Supported DEX Protocols

| Module | DEX Name | Program ID |
|--------|----------|------------|
| `bonkswap` | Bonkswap | `BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p` |
| `boop_fun` | Boop fun | `boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4` |
| `byreal` | Byreal | `REALQqNEomY6cQGZJUGwywTBD2UmDT32rZcNnfxQ5N2` |
| `carrot` | Carrot | `CarrotwivhMpDnm27EHmRLeQ683Z1PufuqEmBZvD282s` |
| `defituna` | DefiTuna | `fUSioN9YKKSa3CUC2YUc4tPkHJ5Y6XW1yz8y6F7qWz9` |
| `dynamic_bonding_curve` | Dynamic Bonding Curve | `dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN` |
| `goosefx_gamma` | GooseFX GAMMA | `GAMMA7meSFWaBXF25oSUgmGRwaW6sCMFLmBNiMSdbHVT` |
| `guacswap` | Guacswap | `Gswppe6ERWKpUTXvRPfXdzHhiCyJvLadVvXGfdpBqcE1` |
| `heaven` | Heaven | `HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o` |
| `helium_network` | Helium Network | `treaf4wWBBty3fHdyBpo35Mz84M8k3heKXmjmi9vFt5` |
| `humidifi` | HumidiFi | `9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp` |
| `metadao` | MetaDAO | `FUTARELBfJfQ8RDGhg1wdhddq1odMAJUePHFuBYfUxKq` |
| `meteora` | Meteora | `Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB` |
| `meteora_damm_v2` | Meteora DAMM v2 | `cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG` |
| `meteora_dlmm` | Meteora DLMM | `LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo` |
| `moonit` | Moonit | `MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG` |
| `openbook_v2` | OpenBook V2 | `opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb` |
| `pancakeswap` | PancakeSwap | `HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq` |
| `perena` | Perena | `NUMERUNsFCP3kuNmWZuXtm1AaQCPj9uw6Guv2Ekoi5P` |
| `perps` | Perps | `PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu` |
| `pump_fun` | Pump fun | `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P` |
| `pump_fun_amm` | Pump fun Amm | `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA` |
| `raydium_amm` | Raydium AMM V4 | `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` |
| `raydium_clmm` | Raydium CLMM | `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK` |
| `raydium_cp` | Raydium CP | `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` |
| `raydium_launchlab` | Raydium Launchlab | `LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj` |
| `saber_decimals` | Saber (Decimals) | `DecZY86MU5Gj7kppfUCEmd4LbXXuyZH1yHaP2NTqdiZB` |
| `solfi_v2` | SolFi V2 | `SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF` |
| `stabble_clmm` | Stabble CLMM | `6dMXqGZ3ga2dikrYS9ovDXgHGh5RUsb2RTUj6hrQXhk6` |
| `stabble_stable_swap` | Stabble Stable Swap | `swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ` |
| `stabble_weighted_swap` | Stabble Weighted Swap | `swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW` |
| `vertigo` | Vertigo | `vrTGoBuy5rYSxAfV3jaRJWHH6nN9WK4NRExGxsk1bCJ` |
| `virtuals` | Virtuals | `5U3EU2ubXtK84QcRjWVmYt9RaDyA8gKxdUrPFXmZyaki` |
| `whirlpool` | Whirlpool | `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc` |
| `woofi` | Woofi | `WooFif76YGRNjk1pA8wCsN67aQsD9f9iLsz4NcJ1AVb` |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dex_pinocchio_cpi = { path = "../dex-pinocchio-cpi" }
```

Or use git dependency:

```toml
[dependencies]
dex_pinocchio_cpi = { git = "https://github.com/vnxfsc/dex-pinocchio-cpi.git" }
```

## Usage

### Basic Usage

Each DEX module provides:

1. **Program ID**: `ID` constant
2. **Instruction Discriminators**: `INSTRUCTION_NAME` constants (8-byte arrays)
3. **Argument Structs**: `InstructionNameArgs` structs
4. **CPI Functions**: `instruction_name()` and `instruction_name_signed()` functions

### Example: Pump.fun Buy

```rust
use dex_pinocchio_cpi::pump_fun::{self, BuyArgs, BuyAccounts};
use pinocchio::{AccountView, ProgramResult, cpi::Signer};

pub fn buy_token<'a>(
    global: &'a AccountView,
    fee_recipient: &'a AccountView,
    mint: &'a AccountView,
    bonding_curve: &'a AccountView,
    associated_bonding_curve: &'a AccountView,
    associated_user: &'a AccountView,
    user: &'a AccountView,
    // ... other accounts
    amount: u64,
    max_sol_cost: u64,
) -> ProgramResult {
    // Build accounts struct
    let accounts = BuyAccounts {
        global,
        fee_recipient,
        mint,
        bonding_curve,
        associated_bonding_curve,
        associated_user,
        user,
        // ... other accounts
    };
    
    // Build arguments
    let args = BuyArgs {
        amount,
        max_sol_cost,
        track_volume: [0u8; 32], // Optional tracking address
    };
    
    // CPI call (empty signers for no PDA signing)
    pump_fun::buy(&accounts, &args, &[])
}

pub fn buy_token_with_pda<'a>(
    accounts: &BuyAccounts<'a>,
    amount: u64,
    max_sol_cost: u64,
    seeds: &[&[u8]],
) -> ProgramResult {
    let args = BuyArgs {
        amount,
        max_sol_cost,
        track_volume: [0u8; 32],
    };
    
    // CPI call with PDA signer
    let signer = Signer::from(seeds);
    pump_fun::buy(accounts, &args, &[signer])
}
```

### Example: Raydium CLMM Swap

```rust
use dex_pinocchio_cpi::raydium_clmm::{self, SwapArgs, SwapAccounts};
use pinocchio::{AccountView, ProgramResult};

pub fn swap<'a>(
    accounts: &SwapAccounts<'a>,
    amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit_x64: u128,
    is_base_input: bool,
) -> ProgramResult {
    let args = SwapArgs {
        amount,
        other_amount_threshold,
        sqrt_price_limit_x64,
        is_base_input,
    };
    
    raydium_clmm::swap(accounts, &args, &[])
}
```

### Get Program ID

```rust
use dex_pinocchio_cpi::pump_fun;
use dex_pinocchio_cpi::raydium_clmm;
use dex_pinocchio_cpi::meteora_dlmm;

// Each module exports an ID constant
let pump_program_id = pump_fun::ID;
let raydium_program_id = raydium_clmm::ID;
let meteora_program_id = meteora_dlmm::ID;
```

### Get Instruction Discriminators

```rust
use dex_pinocchio_cpi::pump_fun::{BUY, SELL, CREATE};

// Discriminators are 8-byte arrays
let buy_discriminator: [u8; 8] = BUY;
let sell_discriminator: [u8; 8] = SELL;
```

## Module Structure

Standard structure for each DEX module:

```rust
// Program ID
pub const ID: Address = ...;

// Instruction discriminators (one per instruction)
pub const INSTRUCTION_NAME: [u8; 8] = [...];

// Argument structs (C layout, directly serializable)
#[repr(C, packed)]
pub struct InstructionNameArgs {
    pub field1: Type1,
    pub field2: Type2,
    // ...
}

// Account structs (type-safe account passing)
pub struct InstructionNameAccounts<'a> {
    pub account1: &'a AccountView,
    pub account2: &'a AccountView,
    // ...
}

// CPI function (unified API with optional signers)
pub fn instruction_name<'a>(
    accounts: &InstructionNameAccounts<'a>,
    args: &InstructionNameArgs,
    signers: &[Signer],
) -> ProgramResult { ... }
```

## Dependencies

```toml
pinocchio = { version = "0.10", features = ["cpi"] }
five8_const = "0.1"
```

## Notes

1. **Account Order**: The `accounts` array must follow the order defined in the IDL
2. **Argument Serialization**: Argument structs use `#[repr(C, packed)]` for direct byte serialization
3. **PDA Signing**: Use `_signed` suffix functions for CPI calls with PDA signers
4. **no_std Environment**: This library has no std dependency and works directly in BPF programs

## License

MIT
