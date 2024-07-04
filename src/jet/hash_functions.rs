/* This file has been automatically generated. */

//! # Hash functions
//!
//! This module defines jets for computing SHA-256 hashes.
//! Be aware that SHA-256 padding isn't provided and messages should be manually padded.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Update the given 256-bit midstate by running the SHA256 block compression function, using the given 512-bit block.
pub fn sha_256_block(a: u256, b: (u256, u256)) -> u256 {
    todo!()
}

/// Add 1   byte  to the SHA256 hash engine.
pub fn sha_256_ctx_8_add_1(a: Ctx8, b: u8) -> Ctx8 {
    todo!()
}

/// Add 2   bytes to the SHA256 hash engine.
pub fn sha_256_ctx_8_add_2(a: Ctx8, b: u16) -> Ctx8 {
    todo!()
}

/// Add 4   bytes to the SHA256 hash engine.
pub fn sha_256_ctx_8_add_4(a: Ctx8, b: u32) -> Ctx8 {
    todo!()
}

/// Add 8   bytes to the SHA256 hash engine.
pub fn sha_256_ctx_8_add_8(a: Ctx8, b: u64) -> Ctx8 {
    todo!()
}

/// Add 16  bytes to the SHA256 hash engine.
pub fn sha_256_ctx_8_add_16(a: Ctx8, b: u128) -> Ctx8 {
    todo!()
}

/// Add 32  bytes to the SHA256 hash engine.
pub fn sha_256_ctx_8_add_32(a: Ctx8, b: u256) -> Ctx8 {
    todo!()
}

/// Add 64  bytes to the SHA256 hash engine.
pub fn sha_256_ctx_8_add_64(a: Ctx8, b: [u8; 64]) -> Ctx8 {
    todo!()
}

/// Add 128 bytes to the SHA256 hash engine.
pub fn sha_256_ctx_8_add_128(a: Ctx8, b: [u8; 128]) -> Ctx8 {
    todo!()
}

/// Add 256 bytes to the SHA256 hash engine.
pub fn sha_256_ctx_8_add_256(a: Ctx8, b: [u8; 256]) -> Ctx8 {
    todo!()
}

/// Add 512 bytes to the SHA256 hash engine.
pub fn sha_256_ctx_8_add_512(a: Ctx8, b: [u8; 512]) -> Ctx8 {
    todo!()
}

/// Add a list of less than 512 bytes to the SHA256 hash engine.
pub fn sha_256_ctx_8_add_buffer_511(a: Ctx8, b: List<u8, 512>) -> Ctx8 {
    todo!()
}

/// Produce a hash from the current state of the SHA256 hash engine.
pub fn sha_256_ctx_8_finalize(a: Ctx8) -> u256 {
    todo!()
}

/// Initialize a default SHA256 hash engine.
pub fn sha_256_ctx_8_init() -> Ctx8 {
    todo!()
}

/// Return the SHA256 initial value.
pub fn sha_256_iv() -> u256 {
    todo!()
}
