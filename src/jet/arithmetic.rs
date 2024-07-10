/* This file has been automatically generated. */

//! # Arithmetic
//!
//! This module defines jets that compute arithmetic functions.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Add two integers and return the carry.
pub fn add_8(a: u8, b: u8) -> (bool, u8) {
    todo!()
}

/// Add two integers and return the carry.
pub fn add_16(a: u16, b: u16) -> (bool, u16) {
    todo!()
}

/// Add two integers and return the carry.
pub fn add_32(a: u32, b: u32) -> (bool, u32) {
    todo!()
}

/// Add two integers and return the carry.
pub fn add_64(a: u64, b: u64) -> (bool, u64) {
    todo!()
}

/// Decrement an integer by one and return the borrow bit.
pub fn decrement_8(a: u8) -> (bool, u8) {
    todo!()
}

/// Decrement an integer by one and return the borrow bit.
pub fn decrement_16(a: u16) -> (bool, u16) {
    todo!()
}

/// Decrement an integer by one and return the borrow bit.
pub fn decrement_32(a: u32) -> (bool, u32) {
    todo!()
}

/// Decrement an integer by one and return the borrow bit.
pub fn decrement_64(a: u64) -> (bool, u64) {
    todo!()
}

/// Divide the first integer by the second integer, and return the remainder.
pub fn div_mod_8(a: u8, b: u8) -> (u8, u8) {
    todo!()
}

/// Divide the first integer by the second integer, and return the remainder.
pub fn div_mod_16(a: u16, b: u16) -> (u16, u16) {
    todo!()
}

/// Divide the first integer by the second integer, and return the remainder.
pub fn div_mod_32(a: u32, b: u32) -> (u32, u32) {
    todo!()
}

/// Divide the first integer by the second integer, and return the remainder.
pub fn div_mod_64(a: u64, b: u64) -> (u64, u64) {
    todo!()
}

/// Divide the first integer by the second integer.
pub fn divide_8(a: u8, b: u8) -> u8 {
    todo!()
}

/// Divide the first integer by the second integer.
pub fn divide_16(a: u16, b: u16) -> u16 {
    todo!()
}

/// Divide the first integer by the second integer.
pub fn divide_32(a: u32, b: u32) -> u32 {
    todo!()
}

/// Divide the first integer by the second integer.
pub fn divide_64(a: u64, b: u64) -> u64 {
    todo!()
}

/// Check if the first integer divides the second integer.
pub fn divides_8(a: u8, b: u8) -> bool {
    todo!()
}

/// Check if the first integer divides the second integer.
pub fn divides_16(a: u16, b: u16) -> bool {
    todo!()
}

/// Check if the first integer divides the second integer.
pub fn divides_32(a: u32, b: u32) -> bool {
    todo!()
}

/// Check if the first integer divides the second integer.
pub fn divides_64(a: u64, b: u64) -> bool {
    todo!()
}

/// Add two integers. Take a carry-in and return a carry-out.
pub fn full_add_8(a: bool, b: (u8, u8)) -> (bool, u8) {
    todo!()
}

/// Add two integers. Take a carry-in and return a carry-out.
pub fn full_add_16(a: bool, b: (u16, u16)) -> (bool, u16) {
    todo!()
}

/// Add two integers. Take a carry-in and return a carry-out.
pub fn full_add_32(a: bool, b: (u32, u32)) -> (bool, u32) {
    todo!()
}

/// Add two integers. Take a carry-in and return a carry-out.
pub fn full_add_64(a: bool, b: (u64, u64)) -> (bool, u64) {
    todo!()
}

/// Decrement an integer by one. Take a borrow-in and return a borrow-out.
pub fn full_decrement_8(a: bool, b: u8) -> (bool, u8) {
    todo!()
}

/// Decrement an integer by one. Take a borrow-in and return a borrow-out.
pub fn full_decrement_16(a: bool, b: u16) -> (bool, u16) {
    todo!()
}

/// Decrement an integer by one. Take a borrow-in and return a borrow-out.
pub fn full_decrement_32(a: bool, b: u32) -> (bool, u32) {
    todo!()
}

/// Decrement an integer by one. Take a borrow-in and return a borrow-out.
pub fn full_decrement_64(a: bool, b: u64) -> (bool, u64) {
    todo!()
}

/// Increment an integer by one. Take a carry-in and return a carry-out.
pub fn full_increment_8(a: bool, b: u8) -> (bool, u8) {
    todo!()
}

/// Increment an integer by one. Take a carry-in and return a carry-out.
pub fn full_increment_16(a: bool, b: u16) -> (bool, u16) {
    todo!()
}

/// Increment an integer by one. Take a carry-in and return a carry-out.
pub fn full_increment_32(a: bool, b: u32) -> (bool, u32) {
    todo!()
}

/// Increment an integer by one. Take a carry-in and return a carry-out.
pub fn full_increment_64(a: bool, b: u64) -> (bool, u64) {
    todo!()
}

/// Helper for multiplying integers. Take the sum of the first pair of integers and add the product of the second pair.
pub fn full_multiply_8(a: (u8, u8), b: (u8, u8)) -> u16 {
    todo!()
}

/// Helper for multiplying integers. Take the sum of the first pair of integers and add the product of the second pair.
pub fn full_multiply_16(a: (u16, u16), b: (u16, u16)) -> u32 {
    todo!()
}

/// Helper for multiplying integers. Take the sum of the first pair of integers and add the product of the second pair.
pub fn full_multiply_32(a: (u32, u32), b: (u32, u32)) -> u64 {
    todo!()
}

/// Helper for multiplying integers. Take the sum of the first pair of integers and add the product of the second pair.
pub fn full_multiply_64(a: (u64, u64), b: (u64, u64)) -> u128 {
    todo!()
}

/// Subtract the second integer from the first integer. Take a borrow-in and return a borrow-out.
pub fn full_subtract_8(a: bool, b: (u8, u8)) -> (bool, u8) {
    todo!()
}

/// Subtract the second integer from the first integer. Take a borrow-in and return a borrow-out.
pub fn full_subtract_16(a: bool, b: (u16, u16)) -> (bool, u16) {
    todo!()
}

/// Subtract the second integer from the first integer. Take a borrow-in and return a borrow-out.
pub fn full_subtract_32(a: bool, b: (u32, u32)) -> (bool, u32) {
    todo!()
}

/// Subtract the second integer from the first integer. Take a borrow-in and return a borrow-out.
pub fn full_subtract_64(a: bool, b: (u64, u64)) -> (bool, u64) {
    todo!()
}

/// Increment an integer by one and return the carry.
pub fn increment_8(a: u8) -> (bool, u8) {
    todo!()
}

/// Increment an integer by one and return the carry.
pub fn increment_16(a: u16) -> (bool, u16) {
    todo!()
}

/// Increment an integer by one and return the carry.
pub fn increment_32(a: u32) -> (bool, u32) {
    todo!()
}

/// Increment an integer by one and return the carry.
pub fn increment_64(a: u64) -> (bool, u64) {
    todo!()
}

/// Check if an integer is one.
pub fn is_one_8(a: u8) -> bool {
    todo!()
}

/// Check if an integer is one.
pub fn is_one_16(a: u16) -> bool {
    todo!()
}

/// Check if an integer is one.
pub fn is_one_32(a: u32) -> bool {
    todo!()
}

/// Check if an integer is one.
pub fn is_one_64(a: u64) -> bool {
    todo!()
}

/// Check if an integer is zero.
pub fn is_zero_8(a: u8) -> bool {
    todo!()
}

/// Check if an integer is zero.
pub fn is_zero_16(a: u16) -> bool {
    todo!()
}

/// Check if an integer is zero.
pub fn is_zero_32(a: u32) -> bool {
    todo!()
}

/// Check if an integer is zero.
pub fn is_zero_64(a: u64) -> bool {
    todo!()
}

/// Check if an integer is less than or equal to another integer.
pub fn le_8(a: u8, b: u8) -> bool {
    todo!()
}

/// Check if an integer is less than or equal to another integer.
pub fn le_16(a: u16, b: u16) -> bool {
    todo!()
}

/// Check if an integer is less than or equal to another integer.
pub fn le_32(a: u32, b: u32) -> bool {
    todo!()
}

/// Check if an integer is less than or equal to another integer.
pub fn le_64(a: u64, b: u64) -> bool {
    todo!()
}

/// Check if an integer is less than another integer.
pub fn lt_8(a: u8, b: u8) -> bool {
    todo!()
}

/// Check if an integer is less than another integer.
pub fn lt_16(a: u16, b: u16) -> bool {
    todo!()
}

/// Check if an integer is less than another integer.
pub fn lt_32(a: u32, b: u32) -> bool {
    todo!()
}

/// Check if an integer is less than another integer.
pub fn lt_64(a: u64, b: u64) -> bool {
    todo!()
}

/// Return the bigger of two integers.
pub fn max_8(a: u8, b: u8) -> u8 {
    todo!()
}

/// Return the bigger of two integers.
pub fn max_16(a: u16, b: u16) -> u16 {
    todo!()
}

/// Return the bigger of two integers.
pub fn max_32(a: u32, b: u32) -> u32 {
    todo!()
}

/// Return the bigger of two integers.
pub fn max_64(a: u64, b: u64) -> u64 {
    todo!()
}

/// Return the median of three integers.
pub fn median_8(a: u8, b: (u8, u8)) -> u8 {
    todo!()
}

/// Return the median of three integers.
pub fn median_16(a: u16, b: (u16, u16)) -> u16 {
    todo!()
}

/// Return the median of three integers.
pub fn median_32(a: u32, b: (u32, u32)) -> u32 {
    todo!()
}

/// Return the median of three integers.
pub fn median_64(a: u64, b: (u64, u64)) -> u64 {
    todo!()
}

/// Return the smaller of two integers.
pub fn min_8(a: u8, b: u8) -> u8 {
    todo!()
}

/// Return the smaller of two integers.
pub fn min_16(a: u16, b: u16) -> u16 {
    todo!()
}

/// Return the smaller of two integers.
pub fn min_32(a: u32, b: u32) -> u32 {
    todo!()
}

/// Return the smaller of two integers.
pub fn min_64(a: u64, b: u64) -> u64 {
    todo!()
}

/// Compute the remainder after dividing both integers.
pub fn modulo_8(a: u8, b: u8) -> u8 {
    todo!()
}

/// Compute the remainder after dividing both integers.
pub fn modulo_16(a: u16, b: u16) -> u16 {
    todo!()
}

/// Compute the remainder after dividing both integers.
pub fn modulo_32(a: u32, b: u32) -> u32 {
    todo!()
}

/// Compute the remainder after dividing both integers.
pub fn modulo_64(a: u64, b: u64) -> u64 {
    todo!()
}

/// Multiply two integers. The output is a 16-bit integer.
pub fn multiply_8(a: u8, b: u8) -> u16 {
    todo!()
}

/// Multiply two integers. The output is a 32-bit integer.
pub fn multiply_16(a: u16, b: u16) -> u32 {
    todo!()
}

/// Multiply two integers. The output is a 64-bit integer.
pub fn multiply_32(a: u32, b: u32) -> u64 {
    todo!()
}

/// Multiply two integers. The output is a 128-bit integer.
pub fn multiply_64(a: u64, b: u64) -> u128 {
    todo!()
}

/// Negate the integer (modulo 2⁸)  and return the borrow bit.
pub fn negate_8(a: u8) -> (bool, u8) {
    todo!()
}

/// Negate the integer (modulo 2¹⁶) and return the borrow bit.
pub fn negate_16(a: u16) -> (bool, u16) {
    todo!()
}

/// Negate the integer (modulo 2³²) and return the borrow bit.
pub fn negate_32(a: u32) -> (bool, u32) {
    todo!()
}

/// Negate the integer (modulo 2⁶⁴) and return the borrow bit.
pub fn negate_64(a: u64) -> (bool, u64) {
    todo!()
}

/// Return 1 as an 8-bit integer.
pub fn one_8() -> u8 {
    todo!()
}

/// Return 1 as a 16-bit integer.
pub fn one_16() -> u16 {
    todo!()
}

/// Return 1 as a 32-bit integer.
pub fn one_32() -> u32 {
    todo!()
}

/// Return 1 as a 64-bit integer.
pub fn one_64() -> u64 {
    todo!()
}

/// Subtract the second integer from the first integer, and return the borrow bit.
pub fn subtract_8(a: u8, b: u8) -> (bool, u8) {
    todo!()
}

/// Subtract the second integer from the first integer, and return the borrow bit.
pub fn subtract_16(a: u16, b: u16) -> (bool, u16) {
    todo!()
}

/// Subtract the second integer from the first integer, and return the borrow bit.
pub fn subtract_32(a: u32, b: u32) -> (bool, u32) {
    todo!()
}

/// Subtract the second integer from the first integer, and return the borrow bit.
pub fn subtract_64(a: u64, b: u64) -> (bool, u64) {
    todo!()
}
