# WASM Semantic Validation Summary

## [Overview](https://webassembly.github.io/spec/core/valid/index.html)

Validation check that a WASM module is well-formed before instantiation. Validity is defined by a type system over the abstract syntax of a module and its contents. For each piece of abstract syntax, there is a typing rule that specifies the constraints that apply to it. The specification only specifies the constraints and instruction sequence to be valid. Algorithm for validation can be found [here](https://webassembly.github.io/spec/core/appendix/algorithm.html#algo-valid). It is also helpful to first read conventions of the WebAssembly Spec [here](https://webassembly.github.io/spec/core/valid/conventions.html).

## [Instruction Validation](https://webassembly.github.io/spec/core/valid/instructions.html)

Instructions are classified by function types that describe how they manipulate the operand stack.

```rust
fn example<T1, T2>(x: T1) -> T2 {
    unimplemented!()
}
```
The types describe the required input stack with argument values of types T1 that an instruction pops off and the provided output stack with result values of types T2 that it pushes back.

For some instructions, the typing rules do not fully constrain the type, and therefore allow for multiple types. Such instructions are called polymorphic. Two degrees of polymorphism can be distinguished:

- value-polymorphic: the value type t of one or several individual operands is unconstrained. That is the case for all parametric instructions like 𝖽𝗋𝗈𝗉 and 𝗌𝖾𝗅𝖾𝖼𝗍.
- stack-polymorphic: the entire (or most of the) function type T1→T2 of the instruction is unconstrained. That is the case for all control instructions that perform an unconditional control transfer, such as 𝗎𝗇𝗋𝖾𝖺𝖼𝗁𝖺𝖻𝗅𝖾, 𝖻𝗋, 𝖻𝗋_𝗍𝖺𝖻𝗅𝖾, and 𝗋𝖾𝗍𝗎𝗋𝗇.

### Numeric Instructions
```rust
fn const<T>() -> T {
    unimplemented!()
}

fn unop<T>(x: T) -> T {
    unimplemented!()
}

fn binop<T>(x1: T, x2: T) -> T {
    unimplemented!()
}

fn testop<T>(x: T) -> i32 {
    unimplemented!()
}

fn relop<T>(x1: T, x2: T) -> i32 {
    unimplemented!()
}

fn cvtop<T1, T2>(x: T1) -> T2 {

}

```
### Parametric Instructions

### Variable Instructions

### Memory Instructions

### Control Instructions

### Instruction Sequences

### Expressions