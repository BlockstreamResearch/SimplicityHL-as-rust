/* This file has been automatically generated. */

//! Transaction
//!
//! This module defines jets to introspect the contents of an Elements transaction.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Return the [`input_amount`] at the [`current_index`].
pub fn current_amount() -> (Asset1, Amount1) {
    todo!()
}

/// Return the [`input_annex_hash`] at th [`current_index`].
pub fn current_annex_hash() -> Option<u256> {
    todo!()
}

/// Return the [`input_asset`] at the [`current_index`].
pub fn current_asset() -> Asset1 {
    todo!()
}

/// Return the index of the current txin.
pub fn current_index() -> u32 {
    todo!()
}

/// Return the [`issuance_asset_amount`] at the [`current_index`].
pub fn current_issuance_asset_amount() -> Option<Amount1> {
    todo!()
}

/// Return the [`issuance_asset_proof`]  at the [`current_index`].
pub fn current_issuance_asset_proof() -> u256 {
    todo!()
}

/// Return the [`issuance_token_amount`] at the [`current_index`].
pub fn current_issuance_token_amount() -> Option<TokenAmount1> {
    todo!()
}

/// Return the [`issuance_token_proof`]  at the [`current_index`].
pub fn current_issuance_token_proof() -> u256 {
    todo!()
}

/// Return the [`new_issuance_contract`] at the [`current_index`].
pub fn current_new_issuance_contract() -> Option<u256> {
    todo!()
}

/// Return the [`input_pegin`] at the [`current_index`].
pub fn current_pegin() -> Option<u256> {
    todo!()
}

/// Return the previous outpoint of the current txin.
pub fn current_prev_outpoint() -> Outpoint {
    todo!()
}

/// Return the [`reissuance_blinding`] at the [`current_index`].
pub fn current_reissuance_blinding() -> Option<ExplicitNonce> {
    todo!()
}

/// Return the [`reissuance_entropy`]  at the [`current_index`].
pub fn current_reissuance_entropy() -> Option<u256> {
    todo!()
}

/// Return the SHA256 hash of the scriptPubKey of the UTXO of the current txin.
pub fn current_script_hash() -> u256 {
    todo!()
}

/// Return the SHA256 hash of the scriptSig of the current txin.
///
/// SegWit UTXOs enforce scriptSig to be the empty string. In such cases, we return the SHA256 hash of the empty string.
pub fn current_script_sig_hash() -> u256 {
    todo!()
}

/// Return the nSequence of the current txin.
///
/// Use this jet to obtain the raw, encoded sequence number.
/// Use [`tx_lock_distance`] to obtain a relative block height, or [`tx_lock_duration`] to obtain a relative UNIX timestamp, in a safe manner.
pub fn current_sequence() -> u32 {
    todo!()
}

/// Return the SHA256 hash of the genesis block.
pub fn genesis_block_hash() -> u256 {
    todo!()
}

/// Return the asset id and the asset amount at the given input index.
///
/// Return `None` if the input does not exist.
pub fn input_amount(a: u32) -> Option<(Asset1, Amount1)> {
    todo!()
}

/// Return the SHA256 hash of the annex at the given input:
/// - Return `Some(Some(x))` if the input has an annex that hashes to `x`.
/// - Return `Some(None`) if the input has no annex.
/// - Return `None` if the input does not exist.
pub fn input_annex_hash(a: u32) -> Option<Option<u256>> {
    todo!()
}

/// Return the asset id of the input at the given index.
///
/// Return `None` if the input does not exist.
pub fn input_asset(a: u32) -> Option<Asset1> {
    todo!()
}

/// Return the parent genesis block hash if the input at the given index is a peg-in.
///
/// - Return `Some(None)` if the input is not a peg-in.
/// - Return `None` if the input does not exist.
pub fn input_pegin(a: u32) -> Option<Option<u256>> {
    todo!()
}

/// Return the previous outpoint of the input at the given index.
///
/// Return `None` if the input does not exist.
pub fn input_prev_outpoint(a: u32) -> Option<Outpoint> {
    todo!()
}

/// Return the SHA256 hash of the scriptPubKey of the UTXO of the input at the given index.
///
/// Return `None` if the input does not exist.
pub fn input_script_hash(a: u32) -> Option<u256> {
    todo!()
}

/// Return the SHA256 hash of the scriptSigKey of the input at the given index.
///
/// Return `None` if the input does not exist.
///
/// SegWit UTXOs enforce scriptSig to be the empty string. In such cases, we return the SHA256 hash of the empty string.
pub fn input_script_sig_hash(a: u32) -> Option<u256> {
    todo!()
}

/// Return the nSequence of the input at the given index.
///
/// Return `None` if the input does not exist.
pub fn input_sequence(a: u32) -> Option<u32> {
    todo!()
}

/// Return the internal key of the current input.
///
/// We assume that Simplicity can be spent in Taproot outputs only, so there always exists an internal key.
pub fn internal_key() -> Pubkey {
    todo!()
}

/// Return the possibly confidential amount of the issuance if the input at the given index has an issuance.
///
/// - Return `Some(None)` if the input does not have an issuance.
/// - Return `None` if the input does not exist.
pub fn issuance_asset_amount(a: u32) -> Option<Option<Amount1>> {
    todo!()
}

/// Return the SHA256 hash of the range proof for the amount of the issuance at the given input index.
///
/// - Return the hash of the empty string if the input does not have an issuance.
/// - Return `None` if the input does not exist.
pub fn issuance_asset_proof(a: u32) -> Option<u256> {
    todo!()
}

/// Return the possibly confidential amount of the reissuance tokens if the input at the given index has an issuance.
///
/// - Return `Some(Some(Right(0)))` if the input is itself a reissuance.
/// - Return `Some(None)` if the input does not have an issuance.
/// - Return `None` if the input does not exist.
pub fn issuance_token_amount(a: u32) -> Option<Option<TokenAmount1>> {
    todo!()
}

/// Return the SHA256 hash of the range proof for the amount of the reissuance tokens at the given input index.
///
/// - Return the hash of the empty string if the input does not have an issuance.
/// - Return `None` if the input does not exist.
pub fn issuance_token_proof(a: u32) -> Option<u256> {
    todo!()
}

/// Return the lock time of the transaction.
pub fn lock_time() -> Lock {
    todo!()
}

/// Return the contract hash for the new issuance at the given input index.
///
/// - Return `Some(None)` if the input does not have a new issuance.
/// - Return `None` if the input does not exist.
pub fn new_issuance_contract(a: u32) -> Option<Option<u256>> {
    todo!()
}

/// Return the number of inputs of the transaction.
pub fn num_inputs() -> u32 {
    todo!()
}

/// Return the number of outputs of the transaction.
pub fn num_outputs() -> u32 {
    todo!()
}

/// Return the asset amount of the output at the given index.
///
/// Return `None` if the output does not exist.
pub fn output_amount(a: u32) -> Option<(Asset1, Amount1)> {
    todo!()
}

/// Return the asset id of the output at the given index.
///
/// Return `None` if the output does not exist.
pub fn output_asset(a: u32) -> Option<Asset1> {
    todo!()
}

/// Check if the output at the given index is a fee output.
///
/// Return `None` if the output does not exist.
pub fn output_is_fee(a: u32) -> Option<bool> {
    todo!()
}

/// Return the nonce of the output at the given index.
///
/// - Return `Some(None)` if the output does not have a nonce.
/// - Return `None` if the output does not exist.
pub fn output_nonce(a: u32) -> Option<Option<Nonce>> {
    todo!()
}

/// Return the `b`-th entry of a null data (`OP_RETURN`) output at index `a`.
///
/// - Return `Some(Some(Right(Right(x-1))))` if the entry is `OP_x` for `x` in the range 1..=16.
/// - Return `Some(Some(Right(Left(0))))` if the entry is `OP_1NEGATE`.
/// - Return `Some(Some(Right(Left(1))))` if the entry is `OP_RESERVED`.
/// - Return `Some(Some(Left((x, hash))))` if the entry is pushed data. `hash` is the SHA256 hash of the data pushed and `x` indicates how the data was pushed:
///     - `x == 0` means the push was an immediate 0 to 75 bytes.
///     - `x == 1` means the push was an `OP_PUSHDATA1`.
///     - `x == 2` means the push was an `OP_PUSHDATA2`.
///     - `x == 3` means the push was an `OP_PUSHDATA4`.
/// - Return `Some(None)` if the null data has fewer than `b` entries.
/// - Return `None` if the output is not a null data output.
///
/// Use this jet to read peg-out data from an output.
pub fn output_null_datum(a: u32, b: u32) -> Option<Option<Either<(u2, u256),Either<u1,u4>>>> {
    todo!()
}

/// Return the SHA256 hash of the range proof of the output at the given index.
///
/// Return `None` if the output does not exist.
pub fn output_range_proof(a: u32) -> Option<u256> {
    todo!()
}

/// Return the SHA256 hash of the scriptPubKey of the output at the given index.
///
/// Return `None` if the output does not exist.
pub fn output_script_hash(a: u32) -> Option<u256> {
    todo!()
}

/// Return the SHA256 hash of the surjection proof of the output at the given index.
///
/// Return `None` if the output does not exist.
pub fn output_surjection_proof(a: u32) -> Option<u256> {
    todo!()
}

/// Return the blinding factor used for the reissuance at the given input index.
///
/// - Return `Some(None)` if the input does not have a reissuance.
/// - Return `None` if the input does not exist.
pub fn reissuance_blinding(a: u32) -> Option<Option<ExplicitNonce>> {
    todo!()
}

/// Return the entropy used for the reissuance at the given input index.
///
/// - Return `Some(None)` if the input does not have a reissuance.
/// - Return `None` if the input does not exist.
pub fn reissuance_entropy(a: u32) -> Option<Option<u256>> {
    todo!()
}

/// Return the CMR of the Simplicity program in the current input.
///
/// This is the CMR of the currently executed Simplicity program.
pub fn script_cmr() -> u256 {
    todo!()
}

/// Return the tap leaf version of the current input.
///
/// We assume that Simplicity can be spent in Taproot outputs only, so there always exists a tap leaf.
pub fn tapleaf_version() -> u8 {
    todo!()
}

/// Return the SHA256 hash of the tap path of the current input.
///
/// We assume that Simplicity can be spent in Taproot outputs only, so there always exists a tap path.
pub fn tappath(a: u8) -> Option<u256> {
    todo!()
}

/// Return the total amount of fees paid to the given asset id.
///
/// Return zero for any asset without fees.
pub fn total_fee(a: ExplicitAsset) -> ExplicitAmount {
    todo!()
}

/// Return the transaction ID.
pub fn transaction_id() -> u256 {
    todo!()
}

/// Return the version number of the transaction.
pub fn version() -> u32 {
    todo!()
}
