# Interface between SimplicityHL as Rust

[![GitHub](https://img.shields.io/badge/github-repo-blue.svg)](https://github.com/blockstreamresearch/simplicityhl-as-rust)
[![crates.io](https://img.shields.io/crates/v/simplicityhl-as-rust.svg)](https://crates.io/crates/simplicityhl-as-rust)
[![docs.rs](https://docs.rs/simplicityhl-as-rust/badge.svg)](https://docs.rs/simplicityhl-as-rust)

The [SimplicityHL](https://github.com/BlockstreamResearch/simplicityhl) language feels very similar to Rust. This crate provides tools to work with SimplicityHL as literal Rust.

## Documentation ✅

We define a Rust type for each SimplicityHL type. We define a Rust function for each SimplicityHL jet. `rustdoc` can be used to generate SimplicityHL documentation.

## Source code conversion 🚧

SimplicityHL code is almost Rust code. For the most part, SimplicityHL lacks advanced features such as generics, so the corresponding syntax is missing.

Because there are only few and predictable differences, we can convert SimplicityHL code to Rust code with a simple static algorithm. This tool hasn't been written yet, but it would enable us to compile SimplicityHL code with the Rust compiler.

This means there would be two paths for a SimplicityHL program:

1. SimplicityHL → Simplicity → Bit Machine
2. SimplicityHL → Rust → any CPU

Using this library as prelude, the SimplicityHL program in (1) that runs on the Bit Machine should behave exactly as the SimplicityHL program in (2) that runs on any CPU.
