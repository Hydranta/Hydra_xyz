use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;

// Placeholder types (these should be implemented based on actual CryptoTypes)
#[derive(Clone, Debug)]
pub struct TransactionOutput; // Define this struct according to your CryptoTypes

#[derive(Clone, Debug)]
pub struct BaseInput; // Define this struct according to your CryptoTypes

#[derive(Clone, Debug)]
pub struct KeyInput; // Define this struct according to your CryptoTypes

#[derive(Clone, Debug)]
pub struct MultisignatureInput; // Define this struct according to your CryptoTypes

#[derive(Clone, Debug)]
pub struct TransactionOutputReferenceDetails; // Define this struct according to your CryptoTypes

#[derive(Clone, Debug)]
pub struct CryptoPublicKey; // Define this struct according to your CryptoTypes

#[derive(Clone, Debug)]
pub struct BinaryArray(pub Vec<u8>);

#[derive(Clone, Debug)]
pub struct CryptoHash(pub Vec<u8>); // Placeholder for Crypto::Hash type

#[derive(Clone, Debug)]
pub struct CryptoSignature; // Define this struct according to your CryptoTypes

// Transaction output details, including the associated global index
pub struct TransactionOutputDetails {
    pub output: TransactionOutput,
    pub global_index: u64,
}

// Base input details, including the input and amount
pub struct BaseInputDetails {
    pub input: BaseInput,
    pub amount: u64,
}

// Key input details, including mixin and associated outputs
pub struct KeyInputDetails {
    pub input: KeyInput,
    pub mixin: u64,
    pub outputs: Vec<TransactionOutputReferenceDetails>,
}

// Multisignature input details, including the output reference
pub struct MultisignatureInputDetails {
    pub input: MultisignatureInput,
    pub output: TransactionOutputReferenceDetails,
}

// Enum to handle various types of transaction inputs
pub enum TransactionInputDetails {
    BaseInputDetails(BaseInputDetails),
    KeyInputDetails(KeyInputDetails),
    MultisignatureInputDetails(MultisignatureInputDetails),
}

// Additional details for transaction extra information (public key, nonce, etc.)
pub struct TransactionExtraDetails {
    pub public_key: CryptoPublicKey,
    pub nonce: BinaryArray,
    pub raw: BinaryArray,
    pub from_address: String,
    pub to_address: Vec<String>,
    pub amount: Vec<String>,
    pub version: String,
}

// Main transaction details struct with all the essential transaction information
pub struct TransactionDetails {
    pub hash: CryptoHash,
    pub size: u64,
    pub fee: u64,
    pub total_inputs_amount: u64,
    pub total_outputs_amount: u64,
    pub mixin: u64,
    pub unlock_time: u64,
    pub timestamp: u64,
    pub payment_id: CryptoHash,
    pub has_payment_id: bool,
    pub in_blockchain: bool,
    pub block_hash: CryptoHash,
    pub block_height: u32,
    pub extra: TransactionExtraDetails,
    pub signatures: Vec<Vec<CryptoSignature>>,
    pub inputs: Vec<TransactionInputDetails>,
    pub outputs: Vec<TransactionOutputDetails>,
    pub from_address: String,
    pub to_address: Vec<String>,
    pub amount: Vec<String>,
}

// Block details struct with information about block version, size, etc.
pub struct BlockDetails {
    pub major_version: u8,
    pub minor_version: u8,
    pub timestamp: u64,
    pub prev_block_hash: CryptoHash,
    pub nonce: u32,
    pub is_orphaned: bool,
    pub height: u32,
    pub hash: CryptoHash,
    pub difficulty: u64,
    pub reward: u64,
    pub base_reward: u64,
    pub block_size: u64,
    pub transactions_cumulative_size: u64,
    pub already_generated_coins: u64,
    pub already_generated_transactions: u64,
    pub size_median: u64,
    pub penalty: f64,
    pub total_fee_amount: u64,
    pub transactions: Vec<TransactionDetails>,
}
