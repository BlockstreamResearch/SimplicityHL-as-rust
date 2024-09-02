/* This file has been automatically generated. */

//! # Elements signature hash modes
//!
//! This module defines jets for computing signature hashes of Elements transactions.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Continue a SHA256 hash with an optional hash by appending the following:
/// - If there is no hash, then the byte `0x00`.
/// - If there is a hash, then the byte `0x01` followed by the given hash (32 bytes).
pub fn annex_hash(a: Ctx8, b: Option<u256>) -> Ctx8 {
    todo!()
}

/// Continue a SHA256 hash with the serialization of a confidential asset followed by the serialization of a amount.
pub fn asset_amount_hash(a: Ctx8, b: Asset1, c: Amount1) -> Ctx8 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - The hash of the ASCII string `TapBranch/elements` (32 bytes).
/// - The lexicographically smaller of the two inputs (32 bytes).
/// - The hash of the ASCII string `TapBranch/elements` again (32 bytes).
/// - The lexicographically larger of the two inputs (32 bytes).
///
/// This builds a taproot from two branches.
pub fn build_tapbranch(a: u256, b: u256) -> u256 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - The hash of the ASCII string `TapBranch/elements` (32 bytes).
/// - The hash of the ASCII string `TapBranch/elements` again (32 bytes).
/// - The lexicographically smaller of the two inputs (32 bytes).
/// - The lexicographically larger of the two inputs (32 bytes).
///
/// This builds a taproot from two branches.
pub fn build_tapleaf_simplicity(a: u256) -> u256 {
    todo!()
}

/// Return the SHA256 hash of the serialization of each input UTXO's asset and amount fields.
pub fn input_amounts_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the concatenation of the following for every input:
/// - If the input has no annex, or isn't a taproot spend, then the byte `0x00`.
/// - If the input has an annex, then the byte `0x01` followed by the SHA256 hash of the annex (32 bytes).
pub fn input_annexes_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - If the input is not a pegin, then the byte `0x00`.
/// - If the input is a pegin, then the byte `0x01` followed by the parent chain's genesis hash (32 bytes).
/// - The input's serialized previous transaction id (32 bytes).
/// - The input's previous transaction index in big endian format (4 bytes).
/// - The input's sequence number in big endian format (4 bytes).
/// - If the input has no annex, or isn't a taproot spend, then the byte `0x00`.
/// - If the input has an annex, then the byte `0x01` followed by the SHA256 hash of the annex (32 bytes).
///
/// Return `None` if the input does not exist.
pub fn input_hash(a: u32) -> Option<u256> {
    todo!()
}

/// Return the SHA256 hash of the concatenation of the following for every input:
/// - If the input is not a pegin, then the byte `0x00`.
/// - If the input is a pegin, then the byte `0x01` followed by the parent chain's genesis hash (32 bytes).
/// - The input's serialized previous transaction id (32 bytes).
/// - The input's previous transaction index in big endian format (4 bytes).
///
/// IMPORTANT: the index is serialized in big endian format rather than little endian format.
pub fn input_outpoints_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the concatenation of the SHA256 hash of each input's scriptSig.
///
/// Note that if an input's UTXO uses segwit, then it's scriptSig will necessarily be the empty string. In
/// such cases we still use the SHA256 hash of the empty string.
pub fn input_script_sigs_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the concatenation of the SHA256 hash of each input UTXO's scriptPubKey.
pub fn input_scripts_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the concatenation of the following for every input:
/// - The input's sequence number in big endian format (4 bytes).
///
/// IMPORTANT, the sequence number is serialized in big endian format rather than little endian format.
pub fn input_sequences_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - The serialization of the input UTXO's asset and amount fields.
/// - The SHA256 hash of the input UTXO's scriptPubKey.
///
/// Return `None` if the input does not exist.
pub fn input_utxo_hash(a: u32) -> Option<u256> {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - The result of [`input_amounts_hash`] (32 bytes).
/// - The result of [`input_scripts_hash`] (32 bytes).
pub fn input_utxos_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - The result of [`input_outpoints_hash`] (32 bytes).
/// - The result of [`input_sequences_hash`] (32 bytes).
/// - The result of [`input_annexes_hash`] (32 bytes).
pub fn inputs_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the concatenation of the following for every input:
/// - If the input has no issuance then two bytes `0x00 0x00`.
/// - If the input is has a new issuance then the byte `0x01` followed by a serialization of the calculated issued
/// asset id (32 bytes) followed by the serialization of the (possibly confidential) issued asset amount (9
/// bytes or 33 bytes).
/// - If the input is has a reissuance then the byte `0x01` followed by a serialization of the issued asset id
/// (32 bytes), followed by the serialization of the (possibly confidential) issued asset amount (9 bytes or
/// 33 bytes).
///
/// IMPORTANT: If there is an issuance but there are no asset issued (i.e. the amount is null) we serialize
/// the vase as the explicit 0 amount, (i.e. `0x01 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00`).
///
/// Note, the issuance asset id is serialized in the same format as an explicit asset id would be.
pub fn issuance_asset_amounts_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the concatenation of the following for every input:
/// - If the input has no issuance then the byte `0x00`.
/// - If the input is has a new issuance then the byte `0x01` followed by 32 `0x00` bytes and the new issuance's
/// contract hash field (32 bytes).
/// - If the input is has reissuance then the byte `0x01` followed by a serializaiton of the reissuance's blinding
/// nonce field (32 bytes) and the reissuance's entropy field (32 bytes).
///
/// Note that if the issuance is a new issuance then the blinding nonce field is 32 `0x00` bytes and new issuance's
/// contract hash.
pub fn issuance_blinding_entropy_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// 1. The asset issuance:
///     - If the input has no issuance then two bytes `0x00 0x00`.
///     - If the input is has a new issuance then the byte `0x01` followed by a serialization of the calculated issued
///     asset id (32 bytes) followed by the serialization of the (possibly confidential) issued asset amount (9 bytes or 33 bytes).
///     - If the input is has a reissuance then the byte `0x01` followed by a serialization of the issued asset id
///     (32 bytes), followed by the serialization of the (possibly confidential) issued asset amount (9 bytes or 33 bytes).
/// 2. The token issuance:
///     - If the input has no issuance then two bytes `0x00 0x00`.
///     - If the input is has a new issuance then the byte `0x01` followed by a serialization of the calculated issued
///     token id (32 bytes) followed by the serialization of the (possibly confidential) issued token amount (9 bytes or 33 bytes).
///     - If the input is has a reissuance then the byte `0x01` followed by a serialization of the issued token id (32 bytes),
///     followed by the serialization of the explicit 0 amount (i.e `0x01 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00`) (9 bytes).
/// 3. The range proofs:
///     - The SHA256 hash of the range proof of the input's issuance asset amount (32 bytes).
///     - The SHA256 hash of the range proof of the input's issuance token amount (32 bytes).
/// 4. The blinding entropy:
///     - If the input has no issuance then the byte `0x00`.
///     - If the input is has a new issuance then the byte `0x01` followed by 32 `0x00` bytes and the new issuance's
///     contract hash field (32 bytes).
///     - If the input is has reissuance then the byte `0x01` followed by a serializaiton of the reissuance's blinding
///     nonce field (32 bytes) and the reissuance's entropy field (32 bytes).
///
/// Return `None` if the input does not exist.
pub fn issuance_hash(a: u32) -> Option<u256> {
    todo!()
}

/// Return the SHA256 hash of the concatenation of the following for every input:
/// - The SHA256 hash of the range proof of the input's issuance asset amount (32 bytes).
/// - The SHA256 hash of the range proof of the input's issuance token amount (32 bytes).
///
/// Note that each the range proof is considered to be the empty string in the case there is no issuance, or if the
/// asset or token amount doesn't exist (i.e is null). The SHA256 hash of the empty string is still used in these
/// cases.
pub fn issuance_range_proofs_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the concatenation of the following for every input:
/// - If the input has no issuance then two bytes `0x00 0x00`.
/// - If the input is has a new issuance then the byte `0x01` followed by a serialization of the calculated issued
/// token id (32 bytes) followed by the serialization of the (possibly confidential) issued token amount (9
/// bytes or 33 bytes).
/// - If the input is has a reissuance then the byte `0x01` followed by a serialization of the issued token id
/// (32 bytes), followed by the serialization of the explicit 0 amount (i.e `0x01 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00`) (9 bytes).
///
/// IMPORTANT: If there is an issuance but there are no tokens issued (i.e. the amount is null) we serialize
/// the vase as the explicit 0 amount, (i.e. `0x01 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00`).
///
/// Note, the issuance token id is serialized in the same format as an explicit asset id would be.
pub fn issuance_token_amounts_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - The result of [`issuance_asset_amounts_hash`] (32 bytes).
/// - The result of [`issuance_token_amounts_hash`] (32 bytes).
/// - The result of [`issuance_range_proofs_hash`] (32 bytes).
/// - The result of [`issuance_blinding_entropy_hash`] (32 bytes).
pub fn issuances_hash() -> u256 {
    todo!()
}

/// Continue the SHA256 hash with the serialization of an optional nonce.
pub fn nonce_hash(a: Ctx8, b: Option<Nonce>) -> Ctx8 {
    todo!()
}

/// Continue the SHA256 hash with an optional pegin and an outpoint by appending the following:
/// - If the input is not a pegin, then the byte `0x00`.
/// - If the input is a pegin, then the byte `0x01` followed by the given parent genesis hash (32 bytes).
/// - The input's previous transaction id (32 bytes).
/// - The input's previous transaction index in big endian format (4 bytes).
pub fn outpoint_hash(a: Ctx8, b: Option<u256>, c: Outpoint) -> Ctx8 {
    todo!()
}

/// Return the SHA256 hash of the serialization of each output's asset and amount fields.
pub fn output_amounts_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - The serialization of the output's asset and amount fields.
/// - The serialization of the output's nonce field.
/// - The SHA256 hash of the output's scriptPubKey.
/// - The SHA256 hash of the output's range proof.
///
/// Return `None` if the output does not exist.
///
/// Note: the result of [`output_surjection_proofs_hash`] is specifically excluded because surjection proofs are dependent on the inputs as well as the output.
pub fn output_hash(a: u32) -> Option<u256> {
    todo!()
}

/// Return the SHA256 hash of the serialization of each output's nonce field.
pub fn output_nonces_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the concatenation of the SHA256 hash of each output's range proof.
///
/// Note that if the output's amount is explicit then the range proof is considered the empty string.
pub fn output_range_proofs_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the concatenation of the SHA256 hash of each output's scriptPubKey.
pub fn output_scripts_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the concatenation of the SHA256 hash of each output's surjection proof.
///
/// Note that if the output's asset is explicit then the surjection proof is considered the empty string.
pub fn output_surjection_proofs_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - The result of [`output_amounts_hash`] (32 bytes).
/// - The result of [`output_nonces_hash`] (32 bytes).
/// - The result of [`output_scripts_hash`] (32 bytes).
/// - The result of [`output_range_proofs_hash`] (32 bytes).
///
/// Note: the result of [`output_surjection_proofs_hash`] is specifically excluded because surjection proofs are dependent on the inputs as well as the output. See also [`tx_hash`].
pub fn outputs_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - The result of [`genesis_block_hash`] (32 bytes).
/// - The result of [`genesis_block_hash`] again (32 bytes).
/// - The result of [`tx_hash`] (32 bytes).
/// - The result of [`tap_env_hash`] (32 bytes).
/// - The result of [`current_index`] (Note: this is in big endian format) (4 bytes).
///
/// Note: the two copies of the [`genesis_block_hash`] values effectively makes this result a BIP-340 style tagged hash.
pub fn sig_all_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - The result of [`tapleaf_hash`] (32 bytes).
/// - The result of [`tappath_hash`] (32 bytes).
/// - The result of [`internal_key`] (32 bytes).
pub fn tap_env_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - The hash of the ASCII string `TapLeaf/elements` (32 bytes).
/// - The hash of the ASCII string `TapLeaf/elements` again (32 bytes).
/// - The result of [`tapleaf_version`] (1 byte).
/// - The byte `0x20` (1 byte).
/// - The result of [`script_cmr`] (32 bytes).
///
/// Note: this matches Element's modified BIP-0341 definition of tapleaf hash.
pub fn tapleaf_hash() -> u256 {
    todo!()
}

/// Return a hash of the current input's control block excluding the leaf version and the taproot internal key.
///
/// Using the notation of BIP-0341, it returns the SHA256 hash of c[33: 33 + 32m].
pub fn tappath_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the following:
/// - The result of [`version`] (Note: this is in big endian format) (4 bytes).
/// - The result of [`tx_lock_time`] (Note: this is in big endian format) (4 bytes).
/// - The result of [`inputs_hash`] (32 bytes).
/// - The result of [`outputs_hash`] (32 bytes).
/// - The result of [`issuances_hash`] (32 bytes).
/// - The result of [`output_surjection_proofs_hash`] (32 bytes).
/// - The result of [`input_utxos_hash`] (32 bytes).
pub fn tx_hash() -> u256 {
    todo!()
}
