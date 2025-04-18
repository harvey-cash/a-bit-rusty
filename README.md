# a-bit-rusty

![Builds](https://github.com/harvey-cash/a-bit-rusty/actions/workflows/build.yml/badge.svg?branch=develop) ![Tests](https://github.com/harvey-cash/a-bit-rusty/actions/workflows/test.yml/badge.svg?branch=develop) ![Coverage](https://img.shields.io/endpoint?style=flat-square&url=https%3A%2F%2Fgist.githubusercontent.com%2Fharvey-cash%2Fba441ef05e0a64327d9a24b5d526d08a%2Fraw%2Fa-bit-rusty-cobertura-coverage.json)

Digital logic library written in Rust.

# Dev Environment Setup (Windows)

- Install [MSVC Prerequisites](https://rust-lang.github.io/rustup/installation/windows-msvc.html) (Windows 11 SDK and MSVC v143 C++ Build Tools)
- Install Rust using [rustup](https://rustup.rs/).
- Install `cargo-llvm-cov`
    ```bash
    cargo install cargo-llvm-cov
    ```
- Run tests with coverage
    ```bash
    cargo llvm-cov
    ```
