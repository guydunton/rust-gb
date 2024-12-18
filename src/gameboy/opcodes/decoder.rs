use super::argument::{arg_from_str, Argument};
use super::category::category_from_str;
use super::cb_opcodes::CB_DICTIONARY;
use super::dictionary::DICTIONARY;
use super::OpCode;

pub enum DecodingError {
    CBFailure,
    DefaultCodeFailure,
}

pub struct Decoder {}

impl Decoder {
    pub fn decode_instruction(program_counter: u16, program_code: &[u8]) -> Result<OpCode, String> {
        let mut code = program_code[program_counter as usize];
        let parts_or_error = match code {
            0xCB => {
                // Get the next code
                code = program_code[program_counter as usize + 1];
                parts_from_dictionary(code, &CB_DICTIONARY, DecodingError::CBFailure)
            }
            _ => {
                // Try to get the value from the dictionary
                parts_from_dictionary(code, &DICTIONARY, DecodingError::DefaultCodeFailure)
            }
        };

        let parts = parts_or_error.map_err(|err_type| match err_type {
            DecodingError::DefaultCodeFailure => format!(
                "Unknown command {:#X} at address: {:#X}",
                code, program_counter
            ),
            DecodingError::CBFailure => format!("Unknown command 0xCB {:#X}", code),
        })?;

        let category = category_from_str(parts[0]);

        let args = parts[1..]
            .iter()
            .map(|arg| arg_from_str(arg, program_counter, program_code));

        let mut clean_args = [Argument::None; 2];

        // Loop through all the arguments and return any errors
        for (i, arg) in args.enumerate() {
            clean_args[i] = arg?;
        }

        Ok(OpCode::new(category, clean_args))
    }
}

fn parts_from_dictionary(
    code: u8,
    dictionary: &'static [(u8, Vec<&'static str>)],
    error: DecodingError,
) -> Result<&'static std::vec::Vec<&'static str>, DecodingError> {
    dictionary
        .iter()
        .find(|(c, _)| *c == code)
        .ok_or(error)
        .map(|(_, parts)| parts)
}
