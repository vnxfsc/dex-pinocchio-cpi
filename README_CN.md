# DEX Pinocchio CPI

[English](./README.md)

Solana DEX 程序的 Pinocchio 兼容 CPI 客户端库，支持 35 个主流 DEX 协议。

## 特性

- **纯 Pinocchio 实现**：极致轻量
- **no_std 兼容**：可用于资源受限环境
- **35 个 DEX 协议**：覆盖 Solana 生态主流 DEX
- **类型安全**：所有指令参数均有严格类型定义

## 项目结构

```
dex-pinocchio-cpi/
├── Cargo.toml              # 项目配置
├── README.md               # 英文文档
├── README_CN.md            # 本文档
├── dex_idls/               # IDL 源文件（34 个 DEX）
│   ├── Pump_fun.json
│   ├── Raydium_CLMM.json
│   ├── Meteora_DLMM.json
│   └── ...
└── src/
    ├── lib.rs              # 库入口，导出所有模块
    ├── pump_fun.rs         # Pump.fun CPI 模块
    ├── raydium_clmm.rs     # Raydium CLMM CPI 模块
    ├── meteora_dlmm.rs     # Meteora DLMM CPI 模块
    └── ...                 # 共 34 个 DEX 模块
```

## 支持的 DEX 协议

| 模块名 | DEX 名称 | Program ID |
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

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
dex_pinocchio_cpi = { path = "../dex-pinocchio-cpi" }
```

或使用 git 依赖：

```toml
[dependencies]
dex_pinocchio_cpi = { git = "https://github.com/vnxfsc/dex-pinocchio-cpi.git" }
```

## 使用方法

### 基本用法

每个 DEX 模块提供以下内容：

1. **Program ID**：`ID` 常量
2. **指令判别器**：`INSTRUCTION_NAME` 常量（8 字节数组）
3. **参数结构体**：`InstructionNameArgs` 结构体
4. **CPI 函数**：`instruction_name()` 和 `instruction_name_signed()` 函数

### 示例：Pump.fun Buy 操作

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
    // ... 其他账户
    amount: u64,
    max_sol_cost: u64,
) -> ProgramResult {
    // 构造账户结构体
    let accounts = BuyAccounts {
        global,
        fee_recipient,
        mint,
        bonding_curve,
        associated_bonding_curve,
        associated_user,
        user,
        // ... 其他账户
    };
    
    // 构造参数
    let args = BuyArgs {
        amount,
        max_sol_cost,
        track_volume: [0u8; 32], // 可选的追踪地址
    };
    
    // 调用 CPI（无 PDA 签名时传空数组）
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
    
    // 带 PDA 签名的 CPI 调用
    let signer = Signer::from(seeds);
    pump_fun::buy(accounts, &args, &[signer])
}
```

### 示例：Raydium CLMM Swap

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

### 获取 Program ID

```rust
use dex_pinocchio_cpi::pump_fun;
use dex_pinocchio_cpi::raydium_clmm;
use dex_pinocchio_cpi::meteora_dlmm;

// 每个模块都导出 ID 常量
let pump_program_id = pump_fun::ID;
let raydium_program_id = raydium_clmm::ID;
let meteora_program_id = meteora_dlmm::ID;
```

### 获取指令判别器

```rust
use dex_pinocchio_cpi::pump_fun::{BUY, SELL, CREATE};

// 判别器是 8 字节数组
let buy_discriminator: [u8; 8] = BUY;
let sell_discriminator: [u8; 8] = SELL;
```

## 模块结构

每个 DEX 模块的标准结构：

```rust
// Program ID
pub const ID: Address = ...;

// 指令判别器（每个指令一个）
pub const INSTRUCTION_NAME: [u8; 8] = [...];

// 参数结构体（C 布局，可直接序列化）
#[repr(C, packed)]
pub struct InstructionNameArgs {
    pub field1: Type1,
    pub field2: Type2,
    // ...
}

// 账户结构体（类型安全的账户传递）
pub struct InstructionNameAccounts<'a> {
    pub account1: &'a AccountView,
    pub account2: &'a AccountView,
    // ...
}

// CPI 函数（统一 API，可选签名者）
pub fn instruction_name<'a>(
    accounts: &InstructionNameAccounts<'a>,
    args: &InstructionNameArgs,
    signers: &[Signer],
) -> ProgramResult { ... }
```

## 依赖

```toml
pinocchio = { version = "0.10", features = ["cpi"] }
five8_const = "0.1"
```

## 注意事项

1. **账户顺序**：传入的 `accounts` 数组必须按照 IDL 中定义的顺序排列
2. **参数序列化**：参数结构体使用 `#[repr(C, packed)]`，可直接作为字节序列化
3. **PDA 签名**：使用 `_signed` 后缀函数进行带 PDA 签名的 CPI 调用
4. **no_std 环境**：本库不依赖 std，可在 BPF 程序中直接使用

## License

MIT
