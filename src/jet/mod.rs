//! Declaration of each jet.
//!
//! Currently for documentation purposes only.

pub mod arithmetic;
pub mod bitcoin_without_primitives;
pub mod digital_signatures;
pub mod elliptic_curve_functions;
pub mod hash_functions;
pub mod issuance;
pub mod multi_bit_logic;
pub mod signature_hash_modes;
pub mod time_locks;
pub mod transaction;

pub use arithmetic::*;
pub use bitcoin_without_primitives::*;
pub use digital_signatures::*;
pub use elliptic_curve_functions::*;
pub use hash_functions::*;
pub use issuance::*;
pub use multi_bit_logic::*;
pub use signature_hash_modes::*;
pub use time_locks::*;
pub use transaction::*;

use either::Either;
use std::marker::PhantomData;

// Phantom declarations to generate documentation.

/// 1-bit unsigned integer.
#[allow(non_camel_case_types)]
pub struct u1;

/// 2-bit unsigned integer.
#[allow(non_camel_case_types)]
pub struct u2;

/// 4-bit unsigned integer.
#[allow(non_camel_case_types)]
pub struct u4;

/// 256-bit unsigned integer.
#[allow(non_camel_case_types)]
pub struct u256;

/// List of less than `BOUND` many values of type `A`.
pub struct List<A, const BOUND: usize> {
    phantom: PhantomData<[A; BOUND]>,
}

/// State of a SHA256 hash engine. SHA context for streams of 8-bit values.
pub type Ctx8 = (List<u8, 64>, (u64, u256));

/// X-only public key.
pub type Pubkey = u256;
/// 256-bit message (signature hash).
pub type Message = u256;
/// 512-bit messaage (CMR of program that computes signature hash + signature hash).
pub type Message64 = [u8; 64];
/// Schnorr signature.
pub type Signature = [u8; 64];

/// Scalar of the secp256k1 elliptic curve.
pub type Scalar = u256;
/// Field element (coordinate) of the secp256k1 elliptic curve.
pub type Fe = u256;
/// Group element (point) of the secp256k1 elliptic curve in affine coordinates.
pub type Ge = (Fe, Fe);
/// Group element (point) of the secp256k1 elliptic curve in projective / Jacobian coordinates.
pub type Gej = (Ge, Fe);
/// Group element (point) of the secp256k1 elliptic curve in compressed affine coordinates
/// (whether y is odd + affine x coordinate).
pub type Point = (u1, Fe);

/// Height of a Bitcoin block.
pub type Height = u32;
/// UNIX timestamp of a Bitcoin block.
pub type Time = u32;
/// Relative distance between Bitcoin blocks in terms of height.
pub type Distance = u16;
/// Relative distance between Bitcoin blocks in terms of UNIX timestamps.
pub type Duration = u16;

/// Lock time of an Elements transaction.
pub type Lock = u32;
/// Outpoint of an Elements transaction input (transaction ID + vout).
pub type Outpoint = (u256, u32);
/// Pedersen commitment to a confidential value.
pub type Confidential1 = Point;
/// Explicit Elements asset ID.
pub type ExplicitAsset = u256;
/// Elements asset (confidential or explicit).
pub type Asset1 = Either<Confidential1, ExplicitAsset>;
/// Explicit amount of units of an Elements asset.
pub type ExplicitAmount = u64;
/// Amount of units of an Elements asset (confidential or explicit).
pub type Amount1 = Either<Confidential1, ExplicitAmount>;
/// Explicit 256-bit nonce.
pub type ExplicitNonce = u256;
/// Nonce (confidential or explicit).
pub type Nonce = Either<Confidential1, ExplicitNonce>;
/// Amount of units of an Elements token (confidential or explicit).
pub type TokenAmount1 = Amount1;
