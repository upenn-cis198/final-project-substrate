# Rust Library for WASM Semantic Validation

## Background

WASM has been viewed as the next generation execution engine for blockchain systems because of its cross platform compatibility and improved performance. Most leading blockchain systems today have moved to a WASM backend. As a result, there will be a proliferation of WASM Smart Contracts in the next few years.

## Problem Statement

As it stands today, there are still some unspecified behaviors when a complier compiles to a WASM binary, especially so for a language not well supported by WASM. Most smart contract execution engine checks for contract validity at run time when the contract is evaluated. This can lead to bug with unspecified behavior or waste of transaction cost.

## Solution

Hence, we are thinking of contributing to an open source [Rust library](https://github.com/wasmx/wasm-chisel) that transforms WASM binaries. Specically, we are looking into semantic validation of a WASM binary described in an [issue](https://github.com/wasmx/wasm-chisel/issues/18) here.

## Steps and Timeline
We will first review and document semantic validation steps needed as per WASM [specification](https://webassembly.github.io/spec/core/valid/index.html). Next, we will review Parity Technologies's Rust-WASM (de)serializer, parity-wasm, to see how much of the validation is implemented. Lastly, we will implement the missing steps in our module. We should approximately spend a week on each step. There is a fair bit of learning curve for WASM itself and parity-wasm.

## Impact
There is indeed a learning curve here for WASM and so does the other project with Substrate. As part of an open source library, this project will be used by future blockchain systems and WASM has many potential use cases for machine learning. Rust is perfect for building such WASM modules given its easy support for WASM.