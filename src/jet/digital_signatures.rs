/* This file has been automatically generated. */

//! # Digital signatures
//!
//! This module defines jets for verifying digital signatures.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Assert that a Schnorr signature matches a public key and message, or panic otherwise.
pub fn bip_0340_verify(a: Pubkey, b: Message, c: Signature) {
    todo!()
}

/// Assert that a Schnorr signature matches a public key and message, using a custom sighash mode. This jet should not be used directly.
pub fn check_sig_verify(a: Pubkey, b: Message64, c: Signature) {
    todo!()
}
