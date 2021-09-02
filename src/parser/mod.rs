//! Parser of the WebAssembly binary format.

pub mod errors;

use crate::Module;
use errors::ParseError;

/// Parses the given bytes into a WebAssembly module.
/// The bytes are parsed using the WebAssembly binary format.
pub fn parse_binary(_bytes: &[u8]) -> Result<Module, ParseError> {
    Ok(Module::empty())
}

/// Parses the given string into a WebAssembly module.
/// The string is first converted to WebAssembly binary, then parse.
/// Some information may be lost in the conversion from text to binary format.
pub fn parse_text(_text: &str) -> Result<Module, ParseError> {
    Ok(Module::empty())
}
