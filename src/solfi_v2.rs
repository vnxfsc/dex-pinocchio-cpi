//! SolFi V2 Pinocchio CPI Client
//!
//! Program ID: SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF
//!
//! SolFi V2 is a native Solana program (non-Anchor) with custom instruction format.
//! This CPI module was reverse-engineered from on-chain transactions.
//!
//! Key findings:
//! - Uses single-byte instruction ID (0x07) instead of 8-byte Anchor discriminator
//! - Swap instruction data is exactly 25 bytes
//! - Requires 13 accounts for swap
//! - Supports Token-2022
//!
//! ## Important Technical Notes
//!
//! ### Slot Validation
//! - Error 0x17: Triggered when account data is stale
//! - Custom(23): Triggered when slot delay exceeds threshold (oracle expired)
//! - Pricing is slot-dependent with non-linear decay (quadratic/exponential)
//! - Higher latency = higher slippage penalty
//!
//! ### CU Consumption
//! - WSOL-USDC swaps may run out of CU when Jupiter routing + WSOL ATA creation exist
//! - Solution: Pre-create WSOL account or use `create_account_with_seed`
//!
//! ### Jupiter Integration
//! - SolFi V2 offers better pricing when called through Jupiter
//! - The `sysvar_instructions` account is used to detect caller (Jupiter vs direct)
//! - Typical price difference: ~0.0289% (up to 0.1085% for some pairs)
//!
//! Supported instructions:
//! - Swap (0x07): Execute token swap

use pinocchio::{
    AccountView, Address, ProgramResult,
    cpi::{invoke_signed, Signer},
    instruction::{InstructionView, InstructionAccount},
};

// ============================================
// Constants
// ============================================

/// SolFi V2 Program ID
pub const PROGRAM_ID: Address = Address::new_from_array(
    five8_const::decode_32_const("SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF")
);

/// Swap instruction ID (single byte, NOT Anchor discriminator)
pub const SWAP_INSTRUCTION_ID: u8 = 0x07;

/// Swap instruction data size
pub const SWAP_DATA_SIZE: usize = 25;

/// Number of accounts required for swap
pub const SWAP_ACCOUNTS_COUNT: usize = 13;

/// SPL Token Program
pub const TOKEN_PROGRAM: Address = Address::new_from_array(
    five8_const::decode_32_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
);

/// SPL Token-2022 Program
pub const TOKEN_2022_PROGRAM: Address = Address::new_from_array(
    five8_const::decode_32_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")
);

// ============================================
// Error Codes (from on-chain analysis)
// ============================================

/// Error: Account data is stale (slot too old)
pub const ERROR_STALE_DATA: u32 = 0x17;

/// Error: Oracle expired (slot delay exceeded threshold)
pub const ERROR_ORACLE_EXPIRED: u32 = 23;

// ============================================
// Market Types (First byte of market_state account)
// ============================================

/// Market type 0xFF - Most common (PUMP, WSOL, ZEC, USD1, etc.)
pub const MARKET_TYPE_FF: u8 = 0xFF;

/// Market type 0xFE - (MON, zenZEC)
pub const MARKET_TYPE_FE: u8 = 0xFE;

/// Market type 0xFD - (HYPE)
pub const MARKET_TYPE_FD: u8 = 0xFD;

/// Market type 0xFC - (USDT)
pub const MARKET_TYPE_FC: u8 = 0xFC;

// ============================================
// Known Active Pool Addresses
// ============================================

/// PUMP-USDC pool (active, type 0xFF)
pub const POOL_PUMP_USDC: Address = Address::new_from_array(
    five8_const::decode_32_const("2kfQuYG2FVZL2RqqKEttcdadbPWP4c7b6AFQztNcBWyV")
);

/// WSOL-USDC pool (active, type 0xFF, high CU consumption)
pub const POOL_WSOL_USDC: Address = Address::new_from_array(
    five8_const::decode_32_const("65ZHSArs5XxPseKQbB1B4r16vDxMWnCxHMzogDAqiDUc")
);

/// USDT-USDC pool (active, type 0xFC)
pub const POOL_USDT_USDC: Address = Address::new_from_array(
    five8_const::decode_32_const("FkEB6uvyzuoaGpgs4yRtFtxC4WJxhejNFbUkj5R6wR32")
);

/// ZEC-USDC pool (active, type 0xFF)
pub const POOL_ZEC_USDC: Address = Address::new_from_array(
    five8_const::decode_32_const("BjBHvbqgQCRmvZ6u3VzGrHn3QZ1NfmMRujoqjeaK6fLT")
);

/// MON-USDC pool (active, type 0xFE)
pub const POOL_MON_USDC: Address = Address::new_from_array(
    five8_const::decode_32_const("2Q6S8p9iZNzMvpTemiC56HqCJ3F3szNoyRkvqEKfCanY")
);

/// HYPE-USDC pool (active, type 0xFD)
pub const POOL_HYPE_USDC: Address = Address::new_from_array(
    five8_const::decode_32_const("2e25gRiddjn968aXrLt1oZw3BZ4fYD5D8mCv7uKxu1yL")
);

/// zenZEC-USDC pool (active, type 0xFE)
pub const POOL_ZENZEC_USDC: Address = Address::new_from_array(
    five8_const::decode_32_const("7TKsqWxU9QkPYVLdjjR1V67ky3FnYogjntUpNLexib4E")
);

// ============================================
// Swap Side Enum
// ============================================

/// Swap side (direction)
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwapSide {
    /// Buy: Quote -> Base (e.g., USDC -> SOL)
    Buy = 0,
    /// Sell: Base -> Quote (e.g., SOL -> USDC)
    Sell = 1,
}

impl SwapSide {
    /// Convert to u64 for instruction data
    #[inline(always)]
    pub const fn to_u64(self) -> u64 {
        self as u64
    }
    
    /// Create from boolean (true = Sell, false = Buy)
    #[inline(always)]
    pub const fn from_is_sell(is_sell: bool) -> Self {
        if is_sell { Self::Sell } else { Self::Buy }
    }
}

// ============================================
// Swap Accounts
// ============================================

/// Swap accounts (13 accounts)
/// 
/// Account structure reverse-engineered from on-chain transactions
pub struct SwapAccounts<'a> {
    /// Market state account (writable)
    /// Contains pool configuration and state
    pub market_state: &'a AccountView,
    
    /// Market authority PDA (readonly)
    /// Signs for vault transfers
    pub authority: &'a AccountView,
    
    /// Base token vault (writable)
    /// Pool's base token reserve
    pub base_vault: &'a AccountView,
    
    /// Quote token vault (writable)
    /// Pool's quote token reserve
    pub quote_vault: &'a AccountView,
    
    /// User's base token account (writable)
    pub user_base_account: &'a AccountView,
    
    /// User's quote token account (writable)
    pub user_quote_account: &'a AccountView,
    
    /// Fee receiver account (writable)
    /// Receives trading fees
    pub fee_receiver: &'a AccountView,
    
    /// Referral account (readonly)
    /// For referral tracking (can be same as fee_receiver)
    pub referral_account: &'a AccountView,
    
    /// Base token mint (readonly)
    pub base_mint: &'a AccountView,
    
    /// Quote token mint (readonly)
    pub quote_mint: &'a AccountView,
    
    /// Token program (readonly)
    /// SPL Token or Token-2022
    pub token_program: &'a AccountView,
    
    /// Second token program (readonly)
    /// For pools with mixed token standards
    pub token_program_2: &'a AccountView,
    
    /// Sysvar instructions (readonly)
    pub sysvar_instructions: &'a AccountView,
}

impl<'a> SwapAccounts<'a> {
    /// Convert to instruction accounts array
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; SWAP_ACCOUNTS_COUNT] {
        [
            // 0. market_state (writable)
            InstructionAccount::writable(self.market_state.address()),
            // 1. authority (readonly)
            InstructionAccount::readonly(self.authority.address()),
            // 2. base_vault (writable)
            InstructionAccount::writable(self.base_vault.address()),
            // 3. quote_vault (writable)
            InstructionAccount::writable(self.quote_vault.address()),
            // 4. user_base_account (writable)
            InstructionAccount::writable(self.user_base_account.address()),
            // 5. user_quote_account (writable)
            InstructionAccount::writable(self.user_quote_account.address()),
            // 6. fee_receiver (writable)
            InstructionAccount::writable(self.fee_receiver.address()),
            // 7. referral_account (readonly)
            InstructionAccount::readonly(self.referral_account.address()),
            // 8. base_mint (readonly)
            InstructionAccount::readonly(self.base_mint.address()),
            // 9. quote_mint (readonly)
            InstructionAccount::readonly(self.quote_mint.address()),
            // 10. token_program (readonly)
            InstructionAccount::readonly(self.token_program.address()),
            // 11. token_program_2 (readonly)
            InstructionAccount::readonly(self.token_program_2.address()),
            // 12. sysvar_instructions (readonly)
            InstructionAccount::readonly(self.sysvar_instructions.address()),
        ]
    }
    
    /// Convert to account views array
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; SWAP_ACCOUNTS_COUNT] {
        [
            self.market_state,
            self.authority,
            self.base_vault,
            self.quote_vault,
            self.user_base_account,
            self.user_quote_account,
            self.fee_receiver,
            self.referral_account,
            self.base_mint,
            self.quote_mint,
            self.token_program,
            self.token_program_2,
            self.sysvar_instructions,
        ]
    }
}

// ============================================
// Swap Arguments
// ============================================

/// Swap instruction arguments
/// 
/// Data layout (25 bytes total):
/// - [0]     instruction_id: u8 = 0x07
/// - [1:9]   amount_in: u64 LE
/// - [9:17]  min_amount_out: u64 LE
/// - [17:25] side: u64 LE (0=Buy, 1=Sell)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct SwapArgs {
    /// Input token amount (raw, with decimals)
    pub amount_in: u64,
    /// Minimum output amount (slippage protection)
    /// Usually set to 0 for market orders
    pub min_amount_out: u64,
    /// Swap direction
    pub side: SwapSide,
}

impl SwapArgs {
    /// Create new swap arguments
    #[inline(always)]
    pub const fn new(amount_in: u64, min_amount_out: u64, side: SwapSide) -> Self {
        Self {
            amount_in,
            min_amount_out,
            side,
        }
    }
    
    /// Create buy swap (Quote -> Base)
    #[inline(always)]
    pub const fn buy(amount_in: u64, min_amount_out: u64) -> Self {
        Self::new(amount_in, min_amount_out, SwapSide::Buy)
    }
    
    /// Create sell swap (Base -> Quote)
    #[inline(always)]
    pub const fn sell(amount_in: u64, min_amount_out: u64) -> Self {
        Self::new(amount_in, min_amount_out, SwapSide::Sell)
    }
    
    /// Serialize to instruction data bytes
    #[inline(always)]
    pub fn to_bytes(&self) -> [u8; SWAP_DATA_SIZE] {
        let mut data = [0u8; SWAP_DATA_SIZE];
        
        // [0] instruction_id
        data[0] = SWAP_INSTRUCTION_ID;
        
        // [1:9] amount_in (u64 LE)
        data[1..9].copy_from_slice(&self.amount_in.to_le_bytes());
        
        // [9:17] min_amount_out (u64 LE)
        data[9..17].copy_from_slice(&self.min_amount_out.to_le_bytes());
        
        // [17:25] side (u64 LE)
        data[17..25].copy_from_slice(&self.side.to_u64().to_le_bytes());
        
        data
    }
}

// ============================================
// Swap Instruction
// ============================================

/// Execute SolFi V2 swap instruction
/// 
/// # Arguments
/// * `accounts` - 13 accounts required for swap
/// * `args` - Swap parameters (amount_in, min_amount_out, side)
/// * `signers` - PDA signers if needed
/// 
/// # Returns
/// * `ProgramResult` - Success or error
/// 
/// # Example
/// ```ignore
/// let args = SwapArgs::buy(1_000_000, 0); // Buy with 1 USDC, no slippage protection
/// swap(&accounts, &args, &[])?;
/// ```
#[inline(always)]
pub fn swap<'a>(
    accounts: &SwapAccounts<'a>,
    args: &SwapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    // Serialize instruction data
    let data = args.to_bytes();
    
    // Build instruction accounts
    let instruction_accounts = accounts.to_instruction_accounts();
    let account_views = accounts.to_views();
    
    // Create instruction
    let instruction = InstructionView {
        program_id: &PROGRAM_ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    // Execute CPI
    invoke_signed::<SWAP_ACCOUNTS_COUNT>(&instruction, &account_views, signers)
}

/// Execute swap with raw parameters (convenience function)
/// 
/// # Arguments
/// * `accounts` - 13 accounts required for swap
/// * `amount_in` - Input token amount
/// * `min_amount_out` - Minimum output amount (0 for no slippage protection)
/// * `is_sell` - true for Sell (Base->Quote), false for Buy (Quote->Base)
/// * `signers` - PDA signers if needed
#[inline(always)]
pub fn swap_raw<'a>(
    accounts: &SwapAccounts<'a>,
    amount_in: u64,
    min_amount_out: u64,
    is_sell: bool,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    let args = SwapArgs::new(
        amount_in,
        min_amount_out,
        SwapSide::from_is_sell(is_sell),
    );
    swap(accounts, &args, signers)
}

// ============================================
// Market State Layout (Partial)
// ============================================

/// Market state account layout
/// 
/// Verified from on-chain data analysis.
/// Account size: 1728 bytes
pub struct MarketStateLayout;

impl MarketStateLayout {
    /// Market type (first byte: 0xFF, 0xFE, 0xFD, 0xFC)
    pub const MARKET_TYPE_OFFSET: usize = 0;
    
    /// Base mint pubkey offset
    pub const BASE_MINT_OFFSET: usize = 8;
    /// Quote mint pubkey offset
    pub const QUOTE_MINT_OFFSET: usize = 40;
    /// Base vault pubkey offset
    pub const BASE_VAULT_OFFSET: usize = 72;
    /// Quote vault pubkey offset
    pub const QUOTE_VAULT_OFFSET: usize = 104;
    /// Fee rate numerator offset
    pub const FEE_RATE_OFFSET: usize = 136;
    
    /// Account size
    pub const SIZE: usize = 1728;
    
    /// Minimum expected size
    pub const MIN_SIZE: usize = 200;
}

/// Parse market type from market state account
#[inline(always)]
pub fn parse_market_type(data: &[u8]) -> Option<u8> {
    if data.is_empty() {
        return None;
    }
    Some(data[MarketStateLayout::MARKET_TYPE_OFFSET])
}

/// Check if market type is valid
#[inline(always)]
pub fn is_valid_market_type(market_type: u8) -> bool {
    matches!(market_type, MARKET_TYPE_FF | MARKET_TYPE_FE | MARKET_TYPE_FD | MARKET_TYPE_FC)
}

/// Parse base vault address from market state
#[inline(always)]
pub fn parse_base_vault(data: &[u8]) -> Option<[u8; 32]> {
    if data.len() < MarketStateLayout::BASE_VAULT_OFFSET + 32 {
        return None;
    }
    
    let mut vault = [0u8; 32];
    vault.copy_from_slice(
        &data[MarketStateLayout::BASE_VAULT_OFFSET..MarketStateLayout::BASE_VAULT_OFFSET + 32]
    );
    Some(vault)
}

/// Parse quote vault address from market state
#[inline(always)]
pub fn parse_quote_vault(data: &[u8]) -> Option<[u8; 32]> {
    if data.len() < MarketStateLayout::QUOTE_VAULT_OFFSET + 32 {
        return None;
    }
    
    let mut vault = [0u8; 32];
    vault.copy_from_slice(
        &data[MarketStateLayout::QUOTE_VAULT_OFFSET..MarketStateLayout::QUOTE_VAULT_OFFSET + 32]
    );
    Some(vault)
}

// ============================================
// Helper Functions
// ============================================

/// Check if a program ID is the SolFi V2 program
#[inline(always)]
pub fn is_solfi_v2_program(program_id: &Address) -> bool {
    program_id == &PROGRAM_ID
}

/// Parse token account balance
/// 
/// SPL Token Account layout:
/// - [0..32] mint
/// - [32..64] owner  
/// - [64..72] amount (u64)
#[inline(always)]
pub fn parse_token_account_balance(data: &[u8]) -> Option<u64> {
    if data.len() < 72 {
        return None;
    }
    
    Some(u64::from_le_bytes(
        data[64..72].try_into().ok()?
    ))
}

/// Get pool reserves from vault accounts
#[inline(always)]
pub fn get_pool_reserves(
    base_vault_data: &[u8],
    quote_vault_data: &[u8],
) -> Option<(u64, u64)> {
    let base_reserve = parse_token_account_balance(base_vault_data)?;
    let quote_reserve = parse_token_account_balance(quote_vault_data)?;
    Some((base_reserve, quote_reserve))
}

/// Calculate expected output for constant product AMM
/// 
/// Formula: output = (reserve_out * amount_in) / (reserve_in + amount_in)
/// 
/// Note: This does not account for fees. Use with caution.
#[inline(always)]
pub fn calculate_output_amount(
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64,
) -> u64 {
    if reserve_in == 0 || reserve_out == 0 || amount_in == 0 {
        return 0;
    }
    
    let numerator = (reserve_out as u128)
        .checked_mul(amount_in as u128)
        .unwrap_or(0);
    
    let denominator = (reserve_in as u128)
        .checked_add(amount_in as u128)
        .unwrap_or(1);
    
    (numerator / denominator) as u64
}

/// Calculate expected output with fee deduction
/// 
/// Default fee assumption: 0.3% (30 bps)
#[inline(always)]
pub fn calculate_output_with_fee(
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64,
    fee_bps: u64,
) -> u64 {
    if reserve_in == 0 || reserve_out == 0 || amount_in == 0 {
        return 0;
    }
    
    // Deduct fee from input
    let fee_multiplier = 10000u64.saturating_sub(fee_bps);
    let amount_in_after_fee = (amount_in as u128)
        .checked_mul(fee_multiplier as u128)
        .unwrap_or(0) / 10000;
    
    let numerator = (reserve_out as u128)
        .checked_mul(amount_in_after_fee)
        .unwrap_or(0);
    
    let denominator = (reserve_in as u128)
        .checked_add(amount_in_after_fee)
        .unwrap_or(1);
    
    (numerator / denominator) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_swap_data_serialization() {
        let args = SwapArgs::sell(1_000_000, 0);
        let data = args.to_bytes();
        
        assert_eq!(data.len(), 25);
        assert_eq!(data[0], 0x07); // instruction_id
        assert_eq!(u64::from_le_bytes(data[1..9].try_into().unwrap()), 1_000_000);
        assert_eq!(u64::from_le_bytes(data[9..17].try_into().unwrap()), 0);
        assert_eq!(u64::from_le_bytes(data[17..25].try_into().unwrap()), 1); // Sell
    }
    
    #[test]
    fn test_swap_side() {
        assert_eq!(SwapSide::Buy.to_u64(), 0);
        assert_eq!(SwapSide::Sell.to_u64(), 1);
        assert_eq!(SwapSide::from_is_sell(false), SwapSide::Buy);
        assert_eq!(SwapSide::from_is_sell(true), SwapSide::Sell);
    }
    
    #[test]
    fn test_output_calculation() {
        // 1000 in, 10000 reserve_in, 10000 reserve_out
        // Expected: 10000 * 1000 / (10000 + 1000) = 909
        let output = calculate_output_amount(1000, 10000, 10000);
        assert_eq!(output, 909);
    }
}
