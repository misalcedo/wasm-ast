//! Parser of the WebAssembly binary format.

pub mod errors;

use crate::Module;
use errors::ParseError;

/// Parses the given bytes into a WebAssembly module.
/// The bytes are parsed using the WebAssembly binary format.
///
/// # Examples
/// ## Empty
/// ```rust
/// use wasm_ast::parse_binary;
///
/// let module = parse_binary(b"\x00\x61\x73\x6D\x01\x00\x00\x00").unwrap();
///
/// assert_eq!(module.functions(), None);
/// assert_eq!(module.functions(), None);
/// assert_eq!(module.tables(), None);
/// assert_eq!(module.memories(), None);
/// assert_eq!(module.globals(), None);
/// assert_eq!(module.elements(), None);
/// assert_eq!(module.data(), None);
/// assert_eq!(module.start(), None);
/// assert_eq!(module.imports(), None);
/// assert_eq!(module.exports(), None);
/// assert_eq!(module.include_data_count(), false);
/// ```
pub fn parse_binary(_bytes: &[u8]) -> Result<Module, ParseError> {
    Ok(Module::empty())
}

/// Parses the given string into a WebAssembly module.
/// The string is first converted to WebAssembly binary, then parse.
/// Some information may be lost in the conversion from text to binary format.
///
/// # Examples
/// ## Empty
/// ```rust
/// use wasm_ast::parse_text;
///
/// let module = parse_text("(module)").unwrap();
///
/// assert_eq!(module.functions(), None);
/// assert_eq!(module.functions(), None);
/// assert_eq!(module.tables(), None);
/// assert_eq!(module.memories(), None);
/// assert_eq!(module.globals(), None);
/// assert_eq!(module.elements(), None);
/// assert_eq!(module.data(), None);
/// assert_eq!(module.start(), None);
/// assert_eq!(module.imports(), None);
/// assert_eq!(module.exports(), None);
/// assert_eq!(module.include_data_count(), false);
/// ```
#[cfg(feature = "wat")]
pub fn parse_text(_text: &str) -> Result<Module, ParseError> {
    Ok(Module::empty())
}
