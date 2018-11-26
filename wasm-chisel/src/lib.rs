use parity_wasm::elements::{External, FunctionType, ImportSection, Module, Type, ValueType};

//pub trait ModuleValidator {
//    fn validate(self, module: &Module) -> Result<bool, String>;
//}
//
//pub struct CheckNumericInstruction {
//	numeric_instruction_verified: bool,
//}
//
//fn check_numeric_instruction(f: Func) -> bool {
//	unimplemented!();
//}
//
//impl ModuleValidator for CheckNumericInstruction {
//	fn validate(self, module: &Module) -> Result<bool, String> {
//		match module.function_section() {
//			Some(functions) => {
//				match functions {
//					Some(function) => check_numeric_instruction(function),
//					None => unimplemented!(),
//				}
//			},
//			None => unimplemented!(),
//		}
//	}
//}

#[cfg(test)]
mod tests {
	use super::*;
	use parity_wasm::elements::deserialize_buffer;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

	#[test]
	fn print_instructions() {
		let wasm: Vec<u8> = vec![
			0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01,
			0x7f, 0x03, 0x02, 0x01, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b,
			0x00, 0x14, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x02, 0x0d, 0x01, 0x00, 0x02, 0x00, 0x03, 0x6c, 0x68,
			0x73, 0x01, 0x03, 0x72, 0x68, 0x73,
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
