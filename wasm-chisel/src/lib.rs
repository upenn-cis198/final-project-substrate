use parity_wasm::elements::*;

pub trait ModuleValidator {
    fn validate(self, module: &Module) -> Result<bool, String>;
}

pub struct CheckNumericInstruction {
	numeric_instruction_verified: bool,
}

fn check_numeric_instuction(Func f) -> bool {
	unimplemented!();
}

impl ModuleValidator for CheckNumericInstruction {
	fn validate(self, module: &Module) -> Result<bool, String> {
		match module.function_section() {
			Some(functions) => {
				match functions {
					Some(function) => check_numeric_instuction(function),
					None => unimplemented()!,
				}
			},
			None => unimplemented()!,
		}
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
