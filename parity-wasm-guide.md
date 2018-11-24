# parity-wasm Navigation Guide

Entry point: 
```rust
let module = parity_wasm::deserialize_file("./res/cases/v1/hello.wasm").unwrap();
assert!(module.code_section().is_some());
```

In [src/elements/mod.rs](https://github.com/paritytech/parity-wasm/blob/8c774ba71d17a72d5d53ecf19c77769b1daa45bc/src/elements/mod.rs#L276-L281), it invokes `deserialize` in [src/elements/module.rs](https://github.com/paritytech/parity-wasm/blob/8c774ba71d17a72d5d53ecf19c77769b1daa45bc/src/elements/module.rs#L381-L430). It then delegates the call to different sections in the binary under [src/elements/section.rs](https://github.com/paritytech/parity-wasm/blob/8c774ba71d17a72d5d53ecf19c77769b1daa45bc/src/elements/section.rs#L82). Module also naively checks for the number of code and function sections [here](https://github.com/paritytech/parity-wasm/blob/8c774ba71d17a72d5d53ecf19c77769b1daa45bc/src/elements/module.rs#L422-L426).

Depending on the reader id from `VarUint7::dserialize(reader)` in `Section::deserialize`, different `deserialize` calls are made as seen [here](https://github.com/paritytech/parity-wasm/blob/8c774ba71d17a72d5d53ecf19c77769b1daa45bc/src/elements/section.rs#L82-L135).

0 - CustomSection::deserialize
1 - TypeSection::deserialize
2 - ImportSection::deserialize
3 - FunctionSection::deserialize
4 - TableSection::deserialize
5 - MemorySection::deserialize
6 - GlobalSection::deserialize
7 - ExportSection::deserialize
8 - Section::Start
9 - ElementSection::deserialize
10 - CodeSection::deserialize
11 - DataSection::deserialize

All of the deserialize functions above take in `read_entries(reader: &mut R)` as an argument. `read_entries` is defined [here](https://github.com/paritytech/parity-wasm/blob/8c774ba71d17a72d5d53ecf19c77769b1daa45bc/src/elements/section.rs#L273-L280) which uses `CountedList::deserialize` which are lists for reading sequence of elements typed T as defined [here](https://github.com/paritytech/parity-wasm/blob/8c774ba71d17a72d5d53ecf19c77769b1daa45bc/src/elements/primitives.rs#L557).

As an example, in `deserialize` above we have, 
```rust
Section::Function(FunctionSection::deserialize(reader)?)
```
`FunctionSection::deserialize` returns 
```rust
Ok(FunctionSection(read_entries(reader)?))
```
and `read_entries` returns a `Result<Vec<T>>`.
