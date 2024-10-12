/* This file has been automatically generated. */

//! # Elliptic curve functions
//!
//! This module defines jets that replicate the functional behavior of (a specific version of) libsecp256k1's elliptic curve operations <https://github.com/bitcoin-core/secp256k1/tree/v0.3.0>.
//! The functions defined here return precisely the same field and point representatives that the corresponding libsecp256k1's functions do, with a few exceptions with the way the point at infinity is handled.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Decompress a point into affine coordinates.
///
/// - Return `None` if the x-coordinate is not on the curve.
/// - Return `Some(ge)` even if the x-coordinate is not normalized.
///
/// ## Cost
///
/// 10861 mWU _(milli weight units)_
pub fn decompress(a: Point) -> Option<Ge> {
    todo!()
}

/// Add two field elements.
///
/// ## Cost
///
/// 755 mWU _(milli weight units)_
pub fn fe_add(a: Fe, b: Fe) -> Fe {
    todo!()
}

/// Compute the modular inverse of a field element.
///
/// ## Cost
///
/// 3175 mWU _(milli weight units)_
pub fn fe_invert(a: Fe) -> Fe {
    todo!()
}

/// Check if the canonical representative of the field element is odd.
///
/// ## Cost
///
/// 290 mWU _(milli weight units)_
pub fn fe_is_odd(a: Fe) -> bool {
    todo!()
}

/// Check if the field element represents zero.
///
/// ## Cost
///
/// 268 mWU _(milli weight units)_
pub fn fe_is_zero(a: Fe) -> bool {
    todo!()
}

/// Multiply two field elements.
///
/// ## Cost
///
/// 808 mWU _(milli weight units)_
pub fn fe_multiply(a: Fe, b: Fe) -> Fe {
    todo!()
}

/// Multiply a field element by the canonical primitive cube root of unity (beta).
///
/// ## Cost
///
/// 579 mWU _(milli weight units)_
pub fn fe_multiply_beta(a: Fe) -> Fe {
    todo!()
}

/// Negate a field element.
///
/// ## Cost
///
/// 531 mWU _(milli weight units)_
pub fn fe_negate(a: Fe) -> Fe {
    todo!()
}

/// Return the canonical representation of a field element.
///
/// ## Cost
///
/// 521 mWU _(milli weight units)_
pub fn fe_normalize(a: Fe) -> Fe {
    todo!()
}

/// Square a field element.
///
/// ## Cost
///
/// 556 mWU _(milli weight units)_
pub fn fe_square(a: Fe) -> Fe {
    todo!()
}

/// Compute the modular square root of a field element if it exists.
///
/// ## Cost
///
/// 10275 mWU _(milli weight units)_
pub fn fe_square_root(a: Fe) -> Option<Fe> {
    todo!()
}

/// Check if the given point satisfies the curve equation y² = x³ + 7.
///
/// ## Cost
///
/// 642 mWU _(milli weight units)_
pub fn ge_is_on_curve(a: Ge) -> bool {
    todo!()
}

/// Negate a point.
///
/// ## Cost
///
/// 945 mWU _(milli weight units)_
pub fn ge_negate(a: Ge) -> Ge {
    todo!()
}

/// Add two points.
///
/// ## Cost
///
/// 2897 mWU _(milli weight units)_
pub fn gej_add(a: Gej, b: Gej) -> Gej {
    todo!()
}

/// Double a point. If the result is the point at infinity, it is returned in canonical form.
///
/// ## Cost
///
/// 1764 mWU _(milli weight units)_
pub fn gej_double(a: Gej) -> Gej {
    todo!()
}

/// Check if two points represent the same point.
///
/// ## Cost
///
/// 2220 mWU _(milli weight units)_
pub fn gej_equiv(a: Gej, b: Gej) -> bool {
    todo!()
}

/// Add two points. If the result is the point at infinity, it is returned in canonical form.
///
/// ## Cost
///
/// 2477 mWU _(milli weight units)_
pub fn gej_ge_add(a: Gej, b: Ge) -> Gej {
    todo!()
}

/// Add two points. Also return the ration of the `a`s z-coordinate and the result's z-coordinate. If the result is the point at infinity, it is returned in canonical form.
///
/// ## Cost
///
/// 2719 mWU _(milli weight units)_
pub fn gej_ge_add_ex(a: Gej, b: Ge) -> (Fe, Gej) {
    todo!()
}

/// Check if two points represent the same point.
///
/// ## Cost
///
/// 1765 mWU _(milli weight units)_
pub fn gej_ge_equiv(a: Gej, b: Ge) -> bool {
    todo!()
}

/// Return the canonical representation of the point at infinity.
///
/// ## Cost
///
/// 716 mWU _(milli weight units)_
pub fn gej_infinity() -> Gej {
    todo!()
}

/// Check if the point represents infinity.
///
/// ## Cost
///
/// 666 mWU _(milli weight units)_
pub fn gej_is_infinity(a: Gej) -> bool {
    todo!()
}

/// Check if the given point satisfies the curve equation y² = x³ + 7.
///
/// ## Cost
///
/// 1016 mWU _(milli weight units)_
pub fn gej_is_on_curve(a: Gej) -> bool {
    todo!()
}

/// Negate a point.
///
/// ## Cost
///
/// 1381 mWU _(milli weight units)_
pub fn gej_negate(a: Gej) -> Gej {
    todo!()
}

/// Convert the point into affine coordinates with canonical field representatives. If the result is the point at infinity, it is returned in canonical form.
///
/// ## Cost
///
/// 4099 mWU _(milli weight units)_
pub fn gej_normalize(a: Gej) -> Option<Ge> {
    todo!()
}

/// Change the representatives of a point by multiplying the z-coefficient by the given value.
///
/// ## Cost
///
/// 1908 mWU _(milli weight units)_
pub fn gej_rescale(a: Gej, b: Fe) -> Gej {
    todo!()
}

/// Check if the point represents an affine point with the given x-coordinate.
///
/// ## Cost
///
/// 1047 mWU _(milli weight units)_
pub fn gej_x_equiv(a: Fe, b: Gej) -> bool {
    todo!()
}

/// Check if the point represents an affine point with odd y-coordinate.
///
/// ## Cost
///
/// 3651 mWU _(milli weight units)_
pub fn gej_y_is_odd(a: Gej) -> bool {
    todo!()
}

/// Multiply the generator point with the given scalar.
///
/// ## Cost
///
/// 50071 mWU _(milli weight units)_
pub fn generate(a: Scalar) -> Gej {
    todo!()
}

/// A cryptographic hash function that results in a point on the secp256k1 curve.
///
/// This matches the hash function used to map asset IDs to asset commitments.
///
/// ## Cost
///
/// 68094 mWU _(milli weight units)_
pub fn hash_to_curve(a: u256) -> Ge {
    todo!()
}

/// Compute the linear combination `b * a + c * g` for point `b` and scalars `a` and `c`, where `g` is the generator point.
///
/// ## Cost
///
/// 84674 mWU _(milli weight units)_
pub fn linear_combination_1(a: (Scalar, Gej), b: Scalar) -> Gej {
    todo!()
}

/// Assert that a point `b` is equal to the linear combination `a.0 * a.1 + a.2 * g`, where `g` is the generator point.
///
/// ## Panics
/// The assertion fails.
///
/// ## Cost
///
/// 43364 mWU _(milli weight units)_
pub fn linear_verify_1(a: ((Scalar, Ge), Scalar), b: Ge) {
    todo!()
}

/// Assert that a point `b` is equal to the linear combination `a.0 * a.1 + a.2 * g`, where `g` is the generator point.
///
/// ## Panics
/// - The assertion fails.
/// - Fails if the points cannot be decompressed.
///
/// ## Cost
///
/// 41494 mWU _(milli weight units)_
pub fn point_verify_1(a: ((Scalar, Point), Scalar), b: Point) {
    todo!()
}

/// Add two scalars.
///
/// ## Cost
///
/// 739 mWU _(milli weight units)_
pub fn scalar_add(a: Scalar, b: Scalar) -> Scalar {
    todo!()
}

/// Compute the modular inverse of a scalar.
///
/// ## Cost
///
/// 3193 mWU _(milli weight units)_
pub fn scalar_invert(a: Scalar) -> Scalar {
    todo!()
}

/// Check if the scalar represents zero.
///
/// ## Cost
///
/// 271 mWU _(milli weight units)_
pub fn scalar_is_zero(a: Scalar) -> bool {
    todo!()
}

/// Multiply two scalars.
///
/// ## Cost
///
/// 774 mWU _(milli weight units)_
pub fn scalar_multiply(a: Scalar, b: Scalar) -> Scalar {
    todo!()
}

/// Multiply a scalar with the canonical primitive cube of unity (lambda)
///
/// ## Cost
///
/// 557 mWU _(milli weight units)_
pub fn scalar_multiply_lambda(a: Scalar) -> Scalar {
    todo!()
}

/// Negate a scalar.
///
/// ## Cost
///
/// 490 mWU _(milli weight units)_
pub fn scalar_negate(a: Scalar) -> Scalar {
    todo!()
}

/// Return the canonical representation of the scalar.
///
/// ## Cost
///
/// 472 mWU _(milli weight units)_
pub fn scalar_normalize(a: Scalar) -> Scalar {
    todo!()
}

/// Square a scalar.
///
/// ## Cost
///
/// 575 mWU _(milli weight units)_
pub fn scalar_square(a: Scalar) -> Scalar {
    todo!()
}

/// Multiply a point by a scalar.
///
/// ## Cost
///
/// 72675 mWU _(milli weight units)_
pub fn scale(a: Scalar, b: Gej) -> Gej {
    todo!()
}

/// Algebraically distribute a field element over the secp256k1 curve as defined in
/// ["Indifferentiable Hashing to Barreto-Naehrig Curves" by Pierre-Alain Fouque, Mehdi Tibouchi](https://inria.hal.science/hal-01094321/file/FT12.pdf).
///
/// While this by itself is not a cryptographic hash function, it can be used as a subroutine
/// in a [`hash_to_curve`] function. However, the distribution only approaches uniformity when it is called twice.
///
/// ## Cost
///
/// 32120 mWU _(milli weight units)_
pub fn swu(a: Fe) -> Ge {
    todo!()
}
