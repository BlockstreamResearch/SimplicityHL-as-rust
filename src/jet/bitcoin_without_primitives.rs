/* This file has been automatically generated. */

//! # Bitcoin (without primitives)
//!
//! This module defines jets for Bitcoin that work without the Bitcoin transaction environmnent.
//! These jets are not recommended for non-Bitcoin(-like) applications.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Parse an integer as a consensus-encoded Bitcoin lock time.
///
/// ## Cost
///
/// 97 mWU _(milli weight units)_
pub fn parse_lock(a: u32) -> Either<Height,Time> {
    todo!()
}

/// Parse an integer as a consensus-encoded Bitcoin sequence number.
///
/// ## Cost
///
/// 116 mWU _(milli weight units)_
pub fn parse_sequence(a: u32) -> Option<Either<Distance,Duration>> {
    todo!()
}

/// Create a SHA256 context, initialized with a "TapData" tag.
///
/// ## Cost
///
/// 1178 mWU _(milli weight units)_
pub fn tapdata_init() -> Ctx8 {
    todo!()
}
