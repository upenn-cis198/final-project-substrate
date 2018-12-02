use parity_wasm::elements::*;

mod classifications;
mod errors;

use crate::classifications::*;
use crate::errors::*;
use self::Filter::*;
use std::mem::discriminant;

pub enum Filter {
	NumericInstructions,
	NoFilter
}

/// Basic struct for validating modules
pub struct ModuleValidator<'a> {
	module: &'a Module,
	filter: Filter,
	stack: Vec<ValueType>
}

impl<'a> ModuleValidator<'a> {

	pub fn new(module: &'a Module, filter: Filter) -> Self {
		ModuleValidator{ module, filter, stack: vec![] }
	}

	pub fn validate(&mut self) -> Result<(), InstructionError> {
		match self.module.code_section() {
			Some(functions) => {
				for (index, function) in functions.bodies().iter().enumerate() {
					self.check_instructions(function, index)?
				}
				Ok(())
			},
			None => Ok(()),
		}
	}

	fn check_instructions(&mut self, body: &FuncBody, index: usize) -> Result<(), InstructionError> {
		for instruction in body.code().elements() {
			println!("{:?}", instruction);
			if GET_INST.iter().any(|f| discriminant(f) == discriminant(instruction)) {
				self.push_get(instruction, body, index)?;
			}
			match self.filter {
				NumericInstructions => {
					if I32_BINOP.iter().any(|f| discriminant(f) == discriminant(instruction)) {
						self.validate_binop(instruction, ValueType::I32)?
					}
				}
				NoFilter => () // TODO: do this
			}
		}
		return Ok(());
	}

	fn validate_binop(&mut self, instruction: &Instruction, vtype: ValueType) -> Result<(), InstructionError> {
		let value1 = self.stack.pop();
		let value2 = self.stack.pop();

		match (value1, value2) {
			(Some(v1), Some(v2)) => {
				if (v1 == vtype && v2 == vtype) {
					self.stack.push(ValueType::I32);
					Ok(())
				} else {
					Err(InstructionError::InvalidBinaryOperation)
				}
			}
			_ => Err(InstructionError::InvalidBinaryOperation)
		}
	}

	fn push_get(&mut self, instruction: &Instruction, body: &FuncBody, index: usize) -> Result<(), InstructionError> {

		// These next couple lines are just to get the parameters of the function we're dealing with.
		// We need the parameters because they can be loaded like local variables but they're not in the locals vec

		// type_ref is the index of the FunctionType in types_section
		let type_ref = self.module.function_section().unwrap().entries()[index].type_ref();
		let type_variant = &self.module.type_section().unwrap().types()[type_ref as usize];

		let mut locals = body.locals().to_vec();
		match type_variant {
			Type::Function(ftype) => {
				locals.extend(ftype.params().iter().map(|f| Local::new(0, *f)));
			}
		}

		match instruction {
			Instruction::GetGlobal(local) => {
				match locals.get(*local as usize) {
					Some(variable) => { 
						self.stack.push(variable.value_type());
						Ok(())					
					},
					None => { Err(InstructionError::GlobalNotFound) },
				}
			},
			Instruction::GetLocal(local) => {
				match locals.get(*local as usize) {
					Some(variable) => {
						self.stack.push(variable.value_type());
						Ok(())
					},
					None => { Err(InstructionError::LocalNotFound) },
				}
			},
			_ => { Err(InstructionError::UnmatchedInstruction) },
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use parity_wasm::elements::deserialize_buffer;

	#[test]
	fn print_instructions_simple_binary() {
		// WAST:
		// (module
		//   (type $t0 (func (param i32 i32) (result i32)))
		//   (func $f0 (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
		//     (i32.add
		//       (get_local $p0)
		//       (get_local $p1))))
		let wasm: Vec<u8> = vec![
			0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01,
			0x7f, 0x03, 0x02, 0x01, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b,
			0x00, 0x14, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x02, 0x0d, 0x01, 0x00, 0x02, 0x00, 0x03, 0x6c, 0x68,
			0x73, 0x01, 0x03, 0x72, 0x68, 0x73
		];

		let module = deserialize_buffer::<Module>(&wasm).unwrap();

		let mut validator = ModuleValidator::new(&module, NumericInstructions);
		validator.validate();
	}

	#[test]
	fn print_instructions_complex_binary() {
		// WAST:
		// (module
		//   (type $t0 (func (param i32 i32) (result i32)))
		//   (func $_Z4multii (export "_Z4multii") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
		//     (i32.mul
		//       (get_local $p1)
		//       (get_local $p0)))
		//   (func $_Z3addii (export "_Z3addii") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
		//     (i32.add
		//       (get_local $p1)
		//       (get_local $p0)))
		//   (func $_Z6divideii (export "_Z6divideii") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
		//     (i32.div_s
		//       (get_local $p0)
		//       (get_local $p1)))
		//   (table $T0 0 anyfunc)
		//   (memory $memory (export "memory") 1))

		let wasm: Vec<u8> = vec![
			0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01,
			0x7f, 0x03, 0x04, 0x03, 0x00, 0x00, 0x00, 0x04, 0x04, 0x01, 0x70, 0x00, 0x00, 0x05, 0x03, 0x01,
			0x00, 0x01, 0x07, 0x2f, 0x04, 0x09, 0x5f, 0x5a, 0x34, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x69, 0x00,
			0x00, 0x08, 0x5f, 0x5a, 0x33, 0x61, 0x64, 0x64, 0x69, 0x69, 0x00, 0x01, 0x0b, 0x5f, 0x5a, 0x36,
			0x64, 0x69, 0x76, 0x69, 0x64, 0x65, 0x69, 0x69, 0x00, 0x02, 0x06, 0x6d, 0x65, 0x6d, 0x6f, 0x72,
			0x79, 0x02, 0x00, 0x0a, 0x19, 0x03, 0x07, 0x00, 0x20, 0x01, 0x20, 0x00, 0x6c, 0x0b, 0x07, 0x00,
			0x20, 0x01, 0x20, 0x00, 0x6a, 0x0b, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6d, 0x0b, 0x00, 0x4b,
			0x04, 0x6e, 0x61, 0x6d, 0x65, 0x01, 0x23, 0x03, 0x00, 0x09, 0x5f, 0x5a, 0x34, 0x6d, 0x75, 0x6c,
			0x74, 0x69, 0x69, 0x01, 0x08, 0x5f, 0x5a, 0x33, 0x61, 0x64, 0x64, 0x69, 0x69, 0x02, 0x0b, 0x5f,
			0x5a, 0x36, 0x64, 0x69, 0x76, 0x69, 0x64, 0x65, 0x69, 0x69, 0x02, 0x1f, 0x03, 0x00, 0x02, 0x00,
			0x02, 0x70, 0x30, 0x01, 0x02, 0x70, 0x31, 0x01, 0x02, 0x00, 0x02, 0x70, 0x30, 0x01, 0x02, 0x70,
			0x31, 0x02, 0x02, 0x00, 0x02, 0x70, 0x30, 0x01, 0x02, 0x70, 0x31
		];

		let module = deserialize_buffer::<Module>(&wasm).unwrap();

		match module.code_section() {
			Some(section) => {
				for function in section.bodies() {
					println!("{:?}", function.code().elements());
				}
			}
			None => println!("No Functions")
		}
	}
}
