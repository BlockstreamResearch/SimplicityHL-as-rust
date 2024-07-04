/* This file has been automatically generated. */

//! # Bitcoin (without primitives)
//!
//! This module defines jets for Bitcoin that work without the Bitcoin transaction environmnent.
//! These jets are not recommended for non-Bitcoin(-like) applications.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Parse an integer as a consensus-encoded Bitcoin lock time.
pub fn parse_lock(a: u32) -> Either<Height,Time> {
    todo!()
}

/// Parse an integer as a consensus-encoded Bitcoin sequence number.
pub fn parse_sequence(a: u32) -> Option<Either<Distance,Duration>> {
    todo!()
}
