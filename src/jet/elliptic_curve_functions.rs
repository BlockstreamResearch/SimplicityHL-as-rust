/* This file has been automatically generated. */

//! # Elliptic curve functions
//!
//! This module defines jets that replicate the functional behavior of (a specific version of) libsecp256k1's elliptic curve operations <https://github.com/bitcoin-core/secp256k1/tree/v0.3.0>.
//! The functions defined here return precisely the same field and point representatives that the corresponding libsecp256k1's functions do, with a few exceptions with the way the point at infinity is handled.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Decompress a point into affine coordinates.  Fails if the x-coordinate is not on the curve, but succeeds even if the x-coordinate is not normalized.
pub fn decompress(a: Point) -> Option<Ge> {
    todo!()
}

/// Add two field elements.
pub fn fe_add(a: Fe, b: Fe) -> Fe {
    todo!()
}

/// Compute the modular inverse of a field element.
pub fn fe_invert(a: Fe) -> Fe {
    todo!()
}

/// Check if the canonical representative of the field element is odd.
pub fn fe_is_odd(a: Fe) -> bool {
    todo!()
}

/// Check if the field element represents zero.
pub fn fe_is_zero(a: Fe) -> bool {
    todo!()
}

/// Multiply two field elements.
pub fn fe_multiply(a: Fe, b: Fe) -> Fe {
    todo!()
}

/// Multiply a field element by the canonical primitive cube root of unity (beta).
pub fn fe_multiply_beta(a: Fe) -> Fe {
    todo!()
}

/// Negate a field element.
pub fn fe_negate(a: Fe) -> Fe {
    todo!()
}

/// Return the canonical representation of a field element.
pub fn fe_normalize(a: Fe) -> Fe {
    todo!()
}

/// Square a field element.
pub fn fe_square(a: Fe) -> Fe {
    todo!()
}

/// Compute the modular square root of a field element if it exists.
pub fn fe_square_root(a: Fe) -> Option<Fe> {
    todo!()
}

/// Check if the given point satisfies the curve equation y² = x³ + 7.
pub fn ge_is_on_curve(a: Ge) -> bool {
    todo!()
}

/// Negate a point.
pub fn ge_negate(a: Ge) -> Ge {
    todo!()
}

/// Add two points.
pub fn gej_add(a: Gej, b: Gej) -> Gej {
    todo!()
}

/// Double a point. If the result is the point at infinity, it is returned in canonical form.
pub fn gej_double(a: Gej) -> Gej {
    todo!()
}

/// Check if two points represent the same point.
pub fn gej_equiv(a: Gej, b: Gej) -> bool {
    todo!()
}

/// Add two points. If the result is the point at infinity, it is returned in canonical form.
pub fn gej_ge_add(a: Gej, b: Ge) -> Gej {
    todo!()
}

/// Add two points. Also return the ration of the `a`s z-coordinate and the result's z-coordinate. If the result is the point at infinity, it is returned in canonical form.
pub fn gej_ge_add_ex(a: Gej, b: Ge) -> (Fe, Gej) {
    todo!()
}

/// Check if two points represent the same point.
pub fn gej_ge_equiv(a: Gej, b: Ge) -> bool {
    todo!()
}

/// Return the canonical representation of the point at infinity.
pub fn gej_infinity() -> Gej {
    todo!()
}

/// Check if the point represents infinity.
pub fn gej_is_infinity(a: Gej) -> bool {
    todo!()
}

/// Check if the given point satisfies the curve equation y² = x³ + 7.
pub fn gej_is_on_curve(a: Gej) -> bool {
    todo!()
}

/// Negate a point.
pub fn gej_negate(a: Gej) -> Gej {
    todo!()
}

/// Convert the point into affine coordinates with canonical field representatives. If the result is the point at infinity, it is returned in canonical form.
pub fn gej_normalize(a: Gej) -> Option<Ge> {
    todo!()
}

/// Change the representatives of a point by multiplying the z-coefficient by the given value.
pub fn gej_rescale(a: Gej, b: Fe) -> Gej {
    todo!()
}

/// Check if the point represents an affine point with the given x-coordinate.
pub fn gej_x_equiv(a: Fe, b: Gej) -> bool {
    todo!()
}

/// Check if the point represents an affine point with odd y-coordinate.
pub fn gej_y_is_odd(a: Gej) -> bool {
    todo!()
}

/// Multiply the generator point with the given scalar.
pub fn generate(a: Scalar) -> Gej {
    todo!()
}

/// Compute the the linear combination `b * a + c * g` for point `b` and scalars `a` and `c`, where `g` is the generator point.
pub fn linear_combination_1(a: (Scalar, Gej), b: Scalar) -> Gej {
    todo!()
}

/// Check if point `b` is equal to the linear combination `a.0 * a.1 + a.2 * g`, where `g` is the generator point.
pub fn linear_verify_1(a: ((Scalar, Ge), Scalar), b: Ge) {
    todo!()
}

/// Check if point `b` is equal to the linear combination `a.0 * a.1 + a.2 * g`, where `g` is the generator point. Fails if the points cannot be decompressed.
pub fn point_verify_1(a: ((Scalar, Point), Scalar), b: Point) {
    todo!()
}

/// Add two scalars.
pub fn scalar_add(a: Scalar, b: Scalar) -> Scalar {
    todo!()
}

/// Compute the modular inverse of a scalar.
pub fn scalar_invert(a: Scalar) -> Scalar {
    todo!()
}

/// Check if the scalar represents zero.
pub fn scalar_is_zero(a: Scalar) -> bool {
    todo!()
}

/// Multiply two scalars.
pub fn scalar_multiply(a: Scalar, b: Scalar) -> Scalar {
    todo!()
}

/// Multiply a scalar with the canonical primitive cube of unity (lambda)
pub fn scalar_multiply_lambda(a: Scalar) -> Scalar {
    todo!()
}

/// Negate a scalar.
pub fn scalar_negate(a: Scalar) -> Scalar {
    todo!()
}

/// Return the canonical representation of the scalar.
pub fn scalar_normalize(a: Scalar) -> Scalar {
    todo!()
}

/// Square a scalar.
pub fn scalar_square(a: Scalar) -> Scalar {
    todo!()
}

/// Multiply a point by a scalar.
pub fn scale(a: Scalar, b: Gej) -> Gej {
    todo!()
}
