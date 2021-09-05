//! Parser of the WebAssembly binary format.

mod errors;
mod instructions;
mod sections;
mod types;
mod values;

use crate::parser::sections::{parse_custom_section, parse_type_section};
use crate::{Module, ModuleSection};
pub use errors::ParseError;
use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
use nom::sequence::tuple;

/// A magic constant used to quickly identify WebAssembly binary file contents.
const PREAMBLE: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];

/// The version of the binary WebAssembly format emitted.
const VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

/// Parses the given bytes into a WebAssembly module.
/// The bytes are parsed using the WebAssembly binary format.
/// Requires that no trailing information is present after the last group of custom sections
/// (i.e. valid WebAssembly binary format passed in with trailing data will be treated as invalid).
///
/// See <https://webassembly.github.io/spec/core/binary/index.html>
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
pub fn parse_binary(input: &[u8]) -> Result<Module, ParseError> {
    let mut builder = Module::builder();

    let (input, _) = tuple((tag(PREAMBLE), tag(VERSION)))(input)?;

    let (input, custom_sections) =
        parse_custom_section(input).map_err(|_| ParseError::InvalidBinary)?;
    builder.set_custom_sections(ModuleSection::Custom, custom_sections);

    let (input, types) = parse_type_section(input)?;
    builder.set_function_types(types);

    let (input, custom_sections) = parse_custom_section(input)?;
    builder.set_custom_sections(ModuleSection::Type, custom_sections);

    let (input, custom_sections) = parse_custom_section(input)?;
    builder.set_custom_sections(ModuleSection::Import, custom_sections);

    let (input, custom_sections) = parse_custom_section(input)?;
    builder.set_custom_sections(ModuleSection::Function, custom_sections);

    let (input, custom_sections) = parse_custom_section(input)?;
    builder.set_custom_sections(ModuleSection::Table, custom_sections);

    let (input, custom_sections) = parse_custom_section(input)?;
    builder.set_custom_sections(ModuleSection::Memory, custom_sections);

    let (input, custom_sections) = parse_custom_section(input)?;
    builder.set_custom_sections(ModuleSection::Global, custom_sections);

    let (input, custom_sections) = parse_custom_section(input)?;
    builder.set_custom_sections(ModuleSection::Export, custom_sections);

    let (input, custom_sections) = parse_custom_section(input)?;
    builder.set_custom_sections(ModuleSection::Start, custom_sections);

    let (input, custom_sections) = parse_custom_section(input)?;
    builder.set_custom_sections(ModuleSection::Element, custom_sections);

    let (input, custom_sections) = parse_custom_section(input)?;
    builder.set_custom_sections(ModuleSection::DataCount, custom_sections);

    let (input, custom_sections) = parse_custom_section(input)?;
    builder.set_custom_sections(ModuleSection::Code, custom_sections);

    let (_, custom_sections) = all_consuming(parse_custom_section)(input)?;
    builder.set_custom_sections(ModuleSection::Data, custom_sections);

    Ok(builder.build())
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
#[cfg(feature = "text")]
pub fn parse_text(text: &str) -> Result<Module, ParseError> {
    let binary = wat::parse_str(text)?;

    parse_binary(binary.as_slice())
}
