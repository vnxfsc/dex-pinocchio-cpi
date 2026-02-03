//! HumidiFi Pinocchio CPI Client
//!
//! Program ID: 9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp
//!
//! HumidiFi is a native Solana program (non-Anchor) with XOR-obfuscated data.
//! This CPI module was reverse-engineered from on-chain transactions.
//!
//! ## Key Features
//!
//! - Uses XOR obfuscation to protect instruction and account data
//! - Two swap versions: Swap (9 accounts) and SwapV2 (13 accounts)
//! - `is_base_to_quote` logic is INVERTED compared to Jupiter!
//! - Account ordering: token_a = quote, token_b = base
//!
//! ## XOR Obfuscation
//!
//! Pool account data and instruction data are XOR-encrypted.
//! Each 32-byte pubkey is split into 4 x 8-byte chunks, each XOR'd with corresponding key.
//!
//! ## Jupiter Integration
//!
//! - HumidiFi (index 87): swap_id, is_base_to_quote
//! - HumidiFiV2 (index 118): swap_id, is_base_to_quote
//!
//! Supported instructions:
//! - Swap: 9 accounts, for basic swaps
//! - SwapV2: 13 accounts, with explicit mints

use pinocchio::{
    AccountView, Address, ProgramResult,
    cpi::{invoke_signed, Signer},
    instruction::{InstructionView, InstructionAccount},
};

// ============================================
// Constants
// ============================================

/// HumidiFi Program ID
pub const PROGRAM_ID: Address = Address::new_from_array(
    five8_const::decode_32_const("9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp")
);

/// Swap instruction data size
pub const SWAP_DATA_SIZE: usize = 25;

/// Swap V1 accounts count
pub const SWAP_V1_ACCOUNTS_COUNT: usize = 9;

/// Swap V2 accounts count
pub const SWAP_V2_ACCOUNTS_COUNT: usize = 13;

/// SPL Token Program
pub const TOKEN_PROGRAM: Address = Address::new_from_array(
    five8_const::decode_32_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
);

/// Clock Sysvar
pub const CLOCK_SYSVAR: Address = Address::new_from_array(
    five8_const::decode_32_const("SysvarC1ock11111111111111111111111111111111")
);

/// Instructions Sysvar
pub const INSTRUCTIONS_SYSVAR: Address = Address::new_from_array(
    five8_const::decode_32_const("Sysvar1nstructions1111111111111111111111111")
);

// ============================================
// XOR Keys for Data Obfuscation
// ============================================

/// XOR keys for decoding pubkeys stored in pool data.
/// Each 32-byte pubkey is split into 4 x 8-byte chunks, each XOR'd with corresponding key.
pub const XOR_KEYS: [u64; 4] = [
    0xfb5c_e87a_ae44_3c38,
    0x04a2_1784_51ba_c3c7,
    0x04a1_1787_51b9_c3c6,
    0x04a0_1786_51b8_c3c5,
];

// ============================================
// XOR Encryption/Decryption Functions
// ============================================

/// Decode a 32-byte XOR-encrypted pubkey from pool data
#[inline(always)]
pub fn xor_decode_pubkey(encrypted: &[u8; 32]) -> [u8; 32] {
    let mut decoded = [0u8; 32];
    
    for i in 0..4 {
        let chunk = u64::from_le_bytes(
            encrypted[i * 8..(i + 1) * 8].try_into().unwrap_or([0u8; 8])
        );
        let dec = chunk ^ XOR_KEYS[i];
        decoded[i * 8..(i + 1) * 8].copy_from_slice(&dec.to_le_bytes());
    }
    
    decoded
}

/// Encode a 32-byte pubkey using XOR encryption
#[inline(always)]
pub fn xor_encode_pubkey(pubkey: &[u8; 32]) -> [u8; 32] {
    // XOR is symmetric, encoding is same as decoding
    xor_decode_pubkey(pubkey)
}

/// Decode a u64 value at specific chunk index
#[inline(always)]
pub fn xor_decode_u64(encrypted: u64, key_index: usize) -> u64 {
    encrypted ^ XOR_KEYS[key_index % 4]
}

/// Encode a u64 value at specific chunk index
#[inline(always)]
pub fn xor_encode_u64(value: u64, key_index: usize) -> u64 {
    value ^ XOR_KEYS[key_index % 4]
}

// ============================================
// Pool Data Layout
// ============================================

/// Pool account data layout
/// 
/// Verified offsets from on-chain analysis:
pub struct PoolDataLayout;

impl PoolDataLayout {
    /// Quote mint (XOR encrypted) - e.g., USDC
    pub const QUOTE_MINT_OFFSET: usize = 384;
    
    /// Base mint (XOR encrypted) - e.g., SOL
    pub const BASE_MINT_OFFSET: usize = 416;
    
    /// Pool account (XOR encrypted)
    pub const POOL_ACCOUNT_OFFSET: usize = 448;
    
    /// Token account (XOR encrypted)
    pub const TOKEN_ACCOUNT_OFFSET: usize = 480;
    
    /// Minimum pool data size
    pub const MIN_SIZE: usize = 512;
}

/// Parse quote mint from pool data
#[inline(always)]
pub fn parse_quote_mint(pool_data: &[u8]) -> Option<[u8; 32]> {
    if pool_data.len() < PoolDataLayout::QUOTE_MINT_OFFSET + 32 {
        return None;
    }
    
    let encrypted: [u8; 32] = pool_data[PoolDataLayout::QUOTE_MINT_OFFSET..PoolDataLayout::QUOTE_MINT_OFFSET + 32]
        .try_into().ok()?;
    
    Some(xor_decode_pubkey(&encrypted))
}

/// Parse base mint from pool data
#[inline(always)]
pub fn parse_base_mint(pool_data: &[u8]) -> Option<[u8; 32]> {
    if pool_data.len() < PoolDataLayout::BASE_MINT_OFFSET + 32 {
        return None;
    }
    
    let encrypted: [u8; 32] = pool_data[PoolDataLayout::BASE_MINT_OFFSET..PoolDataLayout::BASE_MINT_OFFSET + 32]
        .try_into().ok()?;
    
    Some(xor_decode_pubkey(&encrypted))
}

// ============================================
// Swap Direction
// ============================================

/// Swap direction
/// 
/// IMPORTANT: HumidiFi's is_base_to_quote is INVERTED compared to Jupiter!
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwapDirection {
    /// Quote to Base (e.g., USDC -> SOL)
    /// Jupiter: is_base_to_quote = false
    /// HumidiFi Swap: is_base_to_quote = true
    /// HumidiFi SwapV2: is_base_to_quote = false
    QuoteToBase,
    
    /// Base to Quote (e.g., SOL -> USDC)
    /// Jupiter: is_base_to_quote = true
    /// HumidiFi Swap: is_base_to_quote = false
    /// HumidiFi SwapV2: is_base_to_quote = true
    BaseToQuote,
}

impl SwapDirection {
    /// Get the is_base_to_quote value for Swap V1 instruction
    /// NOTE: This is INVERTED from Jupiter's convention!
    #[inline(always)]
    pub const fn to_swap_v1_bool(self) -> bool {
        match self {
            SwapDirection::QuoteToBase => true,   // Jupiter false -> HumidiFi true
            SwapDirection::BaseToQuote => false,  // Jupiter true -> HumidiFi false
        }
    }
    
    /// Get the is_base_to_quote value for Swap V2 instruction
    /// NOTE: Same as Jupiter's convention for V2!
    #[inline(always)]
    pub const fn to_swap_v2_bool(self) -> bool {
        match self {
            SwapDirection::QuoteToBase => false,
            SwapDirection::BaseToQuote => true,
        }
    }
    
    /// Create from Jupiter's is_base_to_quote for Swap V1
    #[inline(always)]
    pub const fn from_jupiter_v1(jupiter_is_base_to_quote: bool) -> Self {
        if jupiter_is_base_to_quote {
            SwapDirection::BaseToQuote
        } else {
            SwapDirection::QuoteToBase
        }
    }
}

// ============================================
// Swap V1 (9 Accounts)
// ============================================

/// Swap V1 accounts (9 accounts)
pub struct SwapV1Accounts<'a> {
    /// User wallet (signer)
    pub user_wallet: &'a AccountView,
    
    /// Pool account (HumidiFi owned, writable)
    pub pool: &'a AccountView,
    
    /// Pool-related account 1 (writable)
    pub pool_account_1: &'a AccountView,
    
    /// Pool-related account 2 (writable)
    pub pool_account_2: &'a AccountView,
    
    /// Pool-related account 3 (writable)
    pub pool_account_3: &'a AccountView,
    
    /// Pool-related account 4 (writable)
    pub pool_account_4: &'a AccountView,
    
    /// Clock sysvar
    pub clock: &'a AccountView,
    
    /// Token program
    pub token_program: &'a AccountView,
    
    /// Instructions sysvar
    pub instructions_sysvar: &'a AccountView,
}

impl<'a> SwapV1Accounts<'a> {
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; SWAP_V1_ACCOUNTS_COUNT] {
        [
            InstructionAccount::readonly_signer(self.user_wallet.address()),
            InstructionAccount::writable(self.pool.address()),
            InstructionAccount::writable(self.pool_account_1.address()),
            InstructionAccount::writable(self.pool_account_2.address()),
            InstructionAccount::writable(self.pool_account_3.address()),
            InstructionAccount::writable(self.pool_account_4.address()),
            InstructionAccount::readonly(self.clock.address()),
            InstructionAccount::readonly(self.token_program.address()),
            InstructionAccount::readonly(self.instructions_sysvar.address()),
        ]
    }
    
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; SWAP_V1_ACCOUNTS_COUNT] {
        [
            self.user_wallet,
            self.pool,
            self.pool_account_1,
            self.pool_account_2,
            self.pool_account_3,
            self.pool_account_4,
            self.clock,
            self.token_program,
            self.instructions_sysvar,
        ]
    }
}

// ============================================
// Swap V2 (13 Accounts)
// ============================================

/// Swap V2 accounts (13 accounts)
pub struct SwapV2Accounts<'a> {
    /// Pool-related account 0 (writable)
    pub pool_account_0: &'a AccountView,
    
    /// Pool-related account 1 (writable)
    pub pool_account_1: &'a AccountView,
    
    /// Pool-related account 2 (writable)
    pub pool_account_2: &'a AccountView,
    
    /// Pool-related account 3 (writable)
    pub pool_account_3: &'a AccountView,
    
    /// Pool-related account 4 (writable)
    pub pool_account_4: &'a AccountView,
    
    /// Pool-related account 5 (writable)
    pub pool_account_5: &'a AccountView,
    
    /// Clock sysvar
    pub clock: &'a AccountView,
    
    /// Token program 1
    pub token_program_1: &'a AccountView,
    
    /// Token program 2 (for Token-2022 support)
    pub token_program_2: &'a AccountView,
    
    /// Instructions sysvar
    pub instructions_sysvar: &'a AccountView,
    
    /// Quote mint (token_a)
    pub quote_mint: &'a AccountView,
    
    /// Base mint (token_b)
    pub base_mint: &'a AccountView,
    
    /// Additional account
    pub additional_account: &'a AccountView,
}

impl<'a> SwapV2Accounts<'a> {
    #[inline(always)]
    pub fn to_instruction_accounts(&self) -> [InstructionAccount<'a>; SWAP_V2_ACCOUNTS_COUNT] {
        [
            InstructionAccount::writable(self.pool_account_0.address()),
            InstructionAccount::writable(self.pool_account_1.address()),
            InstructionAccount::writable(self.pool_account_2.address()),
            InstructionAccount::writable(self.pool_account_3.address()),
            InstructionAccount::writable(self.pool_account_4.address()),
            InstructionAccount::writable(self.pool_account_5.address()),
            InstructionAccount::readonly(self.clock.address()),
            InstructionAccount::readonly(self.token_program_1.address()),
            InstructionAccount::readonly(self.token_program_2.address()),
            InstructionAccount::readonly(self.instructions_sysvar.address()),
            InstructionAccount::readonly(self.quote_mint.address()),
            InstructionAccount::readonly(self.base_mint.address()),
            InstructionAccount::readonly(self.additional_account.address()),
        ]
    }
    
    #[inline(always)]
    pub fn to_views(&self) -> [&'a AccountView; SWAP_V2_ACCOUNTS_COUNT] {
        [
            self.pool_account_0,
            self.pool_account_1,
            self.pool_account_2,
            self.pool_account_3,
            self.pool_account_4,
            self.pool_account_5,
            self.clock,
            self.token_program_1,
            self.token_program_2,
            self.instructions_sysvar,
            self.quote_mint,
            self.base_mint,
            self.additional_account,
        ]
    }
}

// ============================================
// Swap Arguments
// ============================================

/// Swap instruction arguments
/// 
/// NOTE: The actual instruction data is XOR-obfuscated.
/// This struct represents the decoded values.
#[derive(Clone, Copy, Debug)]
pub struct SwapArgs {
    /// Swap ID (identifies the specific pool/pair)
    pub swap_id: u64,
    
    /// Swap direction
    pub direction: SwapDirection,
}

impl SwapArgs {
    /// Create new swap arguments
    #[inline(always)]
    pub const fn new(swap_id: u64, direction: SwapDirection) -> Self {
        Self { swap_id, direction }
    }
    
    /// Serialize to XOR-obfuscated instruction data for Swap V1
    /// 
    /// Data layout (25 bytes):
    /// [0:8]   XOR encrypted data (contains swap_id)
    /// [8:16]  XOR encrypted data
    /// [16]    bit 0 = is_base_to_quote
    /// [17:25] XOR encrypted data
    #[inline(always)]
    pub fn to_bytes_v1(&self) -> [u8; SWAP_DATA_SIZE] {
        let mut data = [0u8; SWAP_DATA_SIZE];
        
        // Encode swap_id into first chunk
        let encoded_swap_id = xor_encode_u64(self.swap_id, 0);
        data[0..8].copy_from_slice(&encoded_swap_id.to_le_bytes());
        
        // Second chunk - placeholder (may need adjustment based on more analysis)
        let encoded_chunk2 = xor_encode_u64(0, 1);
        data[8..16].copy_from_slice(&encoded_chunk2.to_le_bytes());
        
        // Byte 16: bit 0 = is_base_to_quote (inverted for V1)
        // Using observed base value 0x38 (56) with bit 0 for direction
        let base_byte: u8 = 0x38;
        data[16] = if self.direction.to_swap_v1_bool() {
            base_byte | 0x01  // Set bit 0
        } else {
            base_byte & 0xFE  // Clear bit 0
        };
        
        // Last chunk - placeholder
        let encoded_chunk4 = xor_encode_u64(0, 3);
        data[17..25].copy_from_slice(&encoded_chunk4.to_le_bytes());
        
        data
    }
    
    /// Serialize to XOR-obfuscated instruction data for Swap V2
    #[inline(always)]
    pub fn to_bytes_v2(&self) -> [u8; SWAP_DATA_SIZE] {
        let mut data = [0u8; SWAP_DATA_SIZE];
        
        // Similar structure but with V2 direction logic
        let encoded_swap_id = xor_encode_u64(self.swap_id, 0);
        data[0..8].copy_from_slice(&encoded_swap_id.to_le_bytes());
        
        let encoded_chunk2 = xor_encode_u64(0, 1);
        data[8..16].copy_from_slice(&encoded_chunk2.to_le_bytes());
        
        // Byte 16 for V2 (direction not inverted)
        let base_byte: u8 = 0x38;
        data[16] = if self.direction.to_swap_v2_bool() {
            base_byte | 0x01
        } else {
            base_byte & 0xFE
        };
        
        let encoded_chunk4 = xor_encode_u64(0, 3);
        data[17..25].copy_from_slice(&encoded_chunk4.to_le_bytes());
        
        data
    }
}

// ============================================
// Swap Instructions
// ============================================

/// Execute HumidiFi Swap V1 instruction (9 accounts)
/// 
/// # Arguments
/// * `accounts` - 9 accounts required for swap
/// * `args` - Swap parameters (swap_id, direction)
/// * `signers` - PDA signers if needed
/// 
/// # Important
/// - `is_base_to_quote` is INVERTED from Jupiter's convention
/// - token_a = quote, token_b = base
#[inline(always)]
pub fn swap_v1<'a>(
    accounts: &SwapV1Accounts<'a>,
    args: &SwapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    let data = args.to_bytes_v1();
    
    let instruction_accounts = accounts.to_instruction_accounts();
    let account_views = accounts.to_views();
    
    let instruction = InstructionView {
        program_id: &PROGRAM_ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    invoke_signed::<SWAP_V1_ACCOUNTS_COUNT>(&instruction, &account_views, signers)
}

/// Execute HumidiFi Swap V2 instruction (13 accounts)
/// 
/// # Arguments
/// * `accounts` - 13 accounts required for swap
/// * `args` - Swap parameters (swap_id, direction)
/// * `signers` - PDA signers if needed
/// 
/// # Important
/// - V2 direction matches Jupiter's convention
/// - Supports Token-2022
#[inline(always)]
pub fn swap_v2<'a>(
    accounts: &SwapV2Accounts<'a>,
    args: &SwapArgs,
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    let data = args.to_bytes_v2();
    
    let instruction_accounts = accounts.to_instruction_accounts();
    let account_views = accounts.to_views();
    
    let instruction = InstructionView {
        program_id: &PROGRAM_ID,
        accounts: &instruction_accounts,
        data: &data,
    };
    
    invoke_signed::<SWAP_V2_ACCOUNTS_COUNT>(&instruction, &account_views, signers)
}

/// Execute swap with raw obfuscated data
/// 
/// Use this when you already have the XOR-obfuscated instruction data
/// (e.g., copied from a successful transaction)
#[inline(always)]
pub fn swap_v1_raw<'a>(
    accounts: &SwapV1Accounts<'a>,
    raw_data: &[u8; SWAP_DATA_SIZE],
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    let instruction_accounts = accounts.to_instruction_accounts();
    let account_views = accounts.to_views();
    
    let instruction = InstructionView {
        program_id: &PROGRAM_ID,
        accounts: &instruction_accounts,
        data: raw_data,
    };
    
    invoke_signed::<SWAP_V1_ACCOUNTS_COUNT>(&instruction, &account_views, signers)
}

/// Execute swap V2 with raw obfuscated data
#[inline(always)]
pub fn swap_v2_raw<'a>(
    accounts: &SwapV2Accounts<'a>,
    raw_data: &[u8; SWAP_DATA_SIZE],
    signers: &[Signer<'_, '_>],
) -> ProgramResult {
    let instruction_accounts = accounts.to_instruction_accounts();
    let account_views = accounts.to_views();
    
    let instruction = InstructionView {
        program_id: &PROGRAM_ID,
        accounts: &instruction_accounts,
        data: raw_data,
    };
    
    invoke_signed::<SWAP_V2_ACCOUNTS_COUNT>(&instruction, &account_views, signers)
}

// ============================================
// Helper Functions
// ============================================

/// Check if a program ID is HumidiFi
#[inline(always)]
pub fn is_humidifi_program(program_id: &Address) -> bool {
    program_id == &PROGRAM_ID
}

/// Parse token account balance
#[inline(always)]
pub fn parse_token_account_balance(data: &[u8]) -> Option<u64> {
    if data.len() < 72 {
        return None;
    }
    Some(u64::from_le_bytes(data[64..72].try_into().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_xor_symmetric() {
        let original: [u8; 32] = [1u8; 32];
        let encoded = xor_encode_pubkey(&original);
        let decoded = xor_decode_pubkey(&encoded);
        assert_eq!(original, decoded);
    }
    
    #[test]
    fn test_xor_keys() {
        assert_eq!(XOR_KEYS[0], 0xfb5c_e87a_ae44_3c38);
        assert_eq!(XOR_KEYS[1], 0x04a2_1784_51ba_c3c7);
        assert_eq!(XOR_KEYS[2], 0x04a1_1787_51b9_c3c6);
        assert_eq!(XOR_KEYS[3], 0x04a0_1786_51b8_c3c5);
    }
    
    #[test]
    fn test_swap_direction() {
        // V1: direction is inverted
        assert_eq!(SwapDirection::QuoteToBase.to_swap_v1_bool(), true);
        assert_eq!(SwapDirection::BaseToQuote.to_swap_v1_bool(), false);
        
        // V2: direction matches Jupiter
        assert_eq!(SwapDirection::QuoteToBase.to_swap_v2_bool(), false);
        assert_eq!(SwapDirection::BaseToQuote.to_swap_v2_bool(), true);
    }
    
    #[test]
    fn test_swap_data_size() {
        let args = SwapArgs::new(12345, SwapDirection::BaseToQuote);
        let data_v1 = args.to_bytes_v1();
        let data_v2 = args.to_bytes_v2();
        
        assert_eq!(data_v1.len(), 25);
        assert_eq!(data_v2.len(), 25);
    }
}
