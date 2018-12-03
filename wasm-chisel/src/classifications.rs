use parity_wasm::elements::Instruction;
use parity_wasm::elements::Instruction::*;

pub const GET_INST: [Instruction; 2] = [
	GetGlobal(0),
	GetLocal(0),
];

// pub const I32_TESTOP: [Instruction; 1] = [
// 	I32Eqz,
// ];

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

pub const I64_BINOP: [Instruction; 15] = [
	I64Add,
	I64Sub,
	I64Mul,
	I64DivS,
	I64DivU,
	I64RemS,
	I64RemU,
	I64And,
	I64Or,
	I64Xor,
	I64Shl,
	I64ShrS,
	I64ShrU,
	I64Rotl,
	I64Rotr
];

pub const F32_BINOP: [Instruction; 7] = [
	F32Add,
	F32Sub,
	F32Mul,
	F32Div,
	F32Min,
	F32Max,
	F32Copysign,
];

pub const F64_BINOP: [Instruction; 7] = [
	F64Add,
	F64Sub,
	F64Mul,
	F64Div,
	F64Min,
	F64Max,
	F64Copysign,
];