use parity_wasm::elements::Instruction;
use parity_wasm::elements::Instruction::*;

pub const GET_INST: [Instruction; 2] = [
	GetGlobal(0),
	GetLocal(0),
];

pub const I32_BINOP: [Instruction; 15] = [
    I32Add,
	I32Sub,
	I32Mul,
	I32DivS,
	I32DivU,
	I32RemS,
	I32RemU,
	I32And,
	I32Or,
	I32Xor,
	I32Shl,
	I32ShrS,
	I32ShrU,
	I32Rotl,
	I32Rotr,
];