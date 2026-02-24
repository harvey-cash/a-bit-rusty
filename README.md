# a-bit-rusty

![Builds](https://github.com/harvey-cash/a-bit-rusty/actions/workflows/build.yml/badge.svg?branch=develop) ![Tests](https://github.com/harvey-cash/a-bit-rusty/actions/workflows/test.yml/badge.svg?branch=develop) ![Coverage](https://img.shields.io/endpoint?style=flat-square&url=https%3A%2F%2Fgist.githubusercontent.com%2Fharvey-cash%2Fba441ef05e0a64327d9a24b5d526d08a%2Fraw%2Fa-bit-rusty-cobertura-coverage.json)

Digital logic library written in Rust.

## What is it?

**a-bit-rusty** is a digital logic simulation library. It models electronic circuits at the gate level using [NAND gates](https://en.wikipedia.org/wiki/NAND_gate) — a universal logic gate from which every other logic gate can be constructed.

The library lets you define chips, connect them together into circuits, and simulate their behaviour step by step. Circuits can also be compiled (flattened) into reusable chips, enabling hierarchical design.

## Architecture

The library is built around a small set of abstractions in `src/chip/`:

| Module | Description |
|---|---|
| `types.rs` | Core data structures: `NodeType`, `PinLayout`, `ChipAndPin`, `Link`, `LinkMap` |
| `chip.rs` | `Chip` and `Tickable` traits, plus built-in chip implementations |
| `chip_description.rs` | Describes a chip's internal node graph and validates it |
| `circuit.rs` | A collection of interconnected chips that can be ticked (simulated) |
| `circuit_description.rs` | Describes a circuit's structure (chip types and inter-chip links) |
| `compiler.rs` | Compiles a `CircuitDescription` into a flat `ChipDescription` |
| `trace.rs` | `TraceMap` for managing physical wire traces in 3D board space |

### Node types

Every node inside a chip or circuit has one of the following types:

| Type | Role |
|---|---|
| `Ground` | Always outputs `0` |
| `Supply` | Outputs `1` when powered on |
| `Input` | Accepts a value written from outside the chip |
| `Output` | Exposes an internal value to the outside |
| `NAnd` | Computes NAND of its two inputs (`0` only when both inputs are `1`) |
| `Buffer` | Temporary pass-through node used during compilation; removed before simulation |

### Built-in chips

| Chip | Behaviour |
|---|---|
| `GroundChip` | Single output pin, always `0` |
| `SupplyChip` | Single output pin, `1` when on, `0` when off |
| `InputChip` | Single output pin driven by `write_pin` |
| `OutputChip` | Latches its input value on each `tick` |
| `NAndChip` | Two-input NAND gate, built as a `CustomChip` |
| `CustomChip` | User-defined chip described by a `ChipDescription` |

### Simulation

Simulation is tick-based. Calling `tick()` on a `Circuit` (or a `CustomChip`) propagates signal values from inputs through the node graph to outputs using a BFS traversal. Ground and supply nodes are processed first, followed by any inputs that changed since the last tick, ensuring the most recently changed values are prioritised.

### Compiler

`ChipCompiler::compile` takes a `CircuitDescription` and produces a single flat `ChipDescription` that can be used as a `CustomChip` inside another circuit. It:

1. Maps every circuit-level input/output to a node ID.
2. Expands each sub-chip's NAND gates into top-level NAND nodes.
3. Inserts temporary `Buffer` nodes for sub-chip boundaries.
4. Removes buffers with no connections, then "explodes" the remaining buffers (rewires their sources directly to their targets).
5. Passes the resulting node graph to `ChipDescription::new` for final validation.

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
