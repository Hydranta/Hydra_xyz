use serde::{Deserialize, Serialize};
use std::vec::Vec;

/// Enum representing the reasons for transaction removal.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TransactionRemoveReason {
    IncludedInBlock = 0,
    Timeout = 1,
}

/// Details of a "To Key" transaction output.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionOutputToKeyDetails {
    pub tx_out_key: CryptoPublicKey,
}

/// Details of a multisignature transaction output.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionOutputMultisignatureDetails {
    pub keys: Vec<CryptoPublicKey>,
    pub required_signatures: u32,
}

/// Enum representing different transaction output details.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TransactionOutputDetails {
    ToKey(TransactionOutputToKeyDetails),
    Multisignature(TransactionOutputMultisignatureDetails),
}

/// Reference details for a transaction output.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionOutputReferenceDetails {
    pub transaction_hash: CryptoHash,
    pub number: usize,
}

/// Details of a "Generate" transaction input.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionInputGenerateDetails {
    pub height: u32,
}

/// Details of a "To Key" transaction input.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionInputToKeyDetails {
    pub output_indexes: Vec<u32>,
    pub key_image: CryptoKeyImage,
    pub mixin: u64,
    pub outputs: Vec<TransactionOutputReferenceDetails>,
}

/// Details of a multisignature transaction input.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionInputMultisignatureDetails {
    pub signatures: u32,
    pub output: TransactionOutputReferenceDetails,
}

/// Enum representing different transaction input details.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TransactionInputDetails {
    Generate(TransactionInputGenerateDetails),
    ToKey(TransactionInputToKeyDetails),
    Multisignature(TransactionInputMultisignatureDetails),
}

/// Extra details for a transaction.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionExtraDetails {
    pub padding: Vec<usize>,
    pub public_key: Vec<CryptoPublicKey>,
    pub nonce: Vec<String>,
    pub raw: Vec<u8>,
    pub from_address: String,
    pub to_address: Vec<String>,
    pub amount: Vec<String>,
    pub version: String,
}

/// Extended details for a transaction.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionExtraDetails2 {
    pub padding: Vec<usize>,
    pub public_key: CryptoPublicKey,
    pub nonce: BinaryArray,
    pub raw: BinaryArray,
    pub from_address: String,
    pub to_address: Vec<String>,
    pub amount: Vec<String>,
    pub tx_key: CryptoSecretKey,
}

/// Full details of a transaction.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetails {
    pub hash: CryptoHash,
    pub size: u64,
    pub fee: u64,
    pub total_inputs_amount: u64,
    pub total_outputs_amount: u64,
    pub mixin: u64,
    pub unlock_time: u64,
    pub timestamp: u64,
    pub version: u8,
    pub payment_id: CryptoHash,
    pub has_payment_id: bool,
    pub in_blockchain: bool,
    pub block_hash: CryptoHash,
    pub block_height: u32,
    pub extra: TransactionExtraDetails2,
    pub signatures: Vec<Vec<CryptoSignature>>,
    pub inputs: Vec<TransactionInputDetails>,
    pub outputs: Vec<TransactionOutputDetails2>,
    pub from_address: String,
    pub to_address: Vec<String>,
    pub amount: Vec<String>,
    pub tx_key: CryptoSecretKey,
}

/// Details of a blockchain block.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockDetails {
    pub major_version: u8,
    pub minor_version: u8,
    pub timestamp: u64,
    pub prev_block_hash: CryptoHash,
    pub proof_of_work: CryptoHash,
    pub nonce: u32,
    pub is_orphaned: bool,
    pub height: u32,
    pub depth: u32,
    pub hash: CryptoHash,
    pub difficulty: u64,
    pub cumulative_difficulty: u64,
    pub reward: u64,
    pub base_reward: u64,
    pub block_size: u64,
    pub transactions_cumulative_size: u64,
    pub already_generated_coins: u64,
    pub already_generated_transactions: u64,
    pub size_median: u64,
    pub effective_size_median: u64,
    pub penalty: f64,
    pub total_fee_amount: u64,
    pub transactions: Vec<TransactionDetails>,
}

// Aliases for external dependencies
pub type CryptoHash = Vec<u8>;
pub type CryptoPublicKey = Vec<u8>;
pub type CryptoSecretKey = Vec<u8>;
pub type CryptoSignature = Vec<u8>;
pub type CryptoKeyImage = Vec<u8>;
pub type BinaryArray = Vec<u8>;

// Placeholder structs for external dependencies
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionOutput;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BaseInput;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyInput;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultisignatureInput;
