# Interface between Simfony as Rust

[![GitHub](https://img.shields.io/badge/github-repo-blue.svg)](https://github.com/uncomputable/simfony-as-rust)
[![crates.io](https://img.shields.io/crates/v/konbaato.svg)](https://crates.io/crates/simfony-as-rust)
[![docs.rs](https://docs.rs/konbaato/badge.svg)](https://docs.rs/simfony-as-rust)

The [Simfony](https://github.com/BlockstreamResearch/simfony) language feels very similar to Rust. This crate provides tools to work with Simfony as literal Rust.

## Documentation ✅

We define a Rust type for each Simfony type. We define a Rust function for each Simfony jet. `rustdoc` can be used to generate Simfony documentation.

## Source code conversion 🚧

Simfony code is almost Rust code. For the most part, Simfony lacks advanced features such as generics, so the corresponding syntax is missing.

Because there are only few and predictable differences, we can convert Simfony code to Rust code with a simple static algorithm. This tool hasn't been written yet, but it would enable us to compile Simfony code with the Rust compiler.

This means there would be two paths for a Simfony program:

1. Simfony → Simplicity → Bit Machine
2. Simfony → Rust → any CPU

Using this library as prelude, the Simfony program in (1) that runs on the Bit Machine should behave exactly as the Simfony program in (2) that runs on any CPU.
