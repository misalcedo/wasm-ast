use crate::parser::instructions::parse_expression;
use crate::parser::types::{parse_global_type, parse_memory_type, parse_table_type};
use crate::parser::values::{match_byte, parse_name, parse_u32};
use crate::{Global, Import, ImportDescription, Memory, Table};
use nom::branch::alt;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses a WebAssembly import component from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html#binary-importsec>
pub fn parse_import(input: &[u8]) -> IResult<&[u8], Import> {
    map(
        tuple((parse_name, parse_name, parse_import_description)),
        |(module, import, description)| Import::new(module, import, description),
    )(input)
}

/// Parses an import description.
fn parse_import_description(input: &[u8]) -> IResult<&[u8], ImportDescription> {
    alt((
        map(
            preceded(match_byte(0x00), parse_u32),
            ImportDescription::Function,
        ),
        map(
            preceded(match_byte(0x01), parse_table_type),
            ImportDescription::Table,
        ),
        map(
            preceded(match_byte(0x02), parse_memory_type),
            ImportDescription::Memory,
        ),
        map(
            preceded(match_byte(0x03), parse_global_type),
            ImportDescription::Global,
        ),
    ))(input)
}

/// Parses a WebAssembly table component from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html#memory-section>
pub fn parse_table(input: &[u8]) -> IResult<&[u8], Table> {
    map(parse_table_type, Table::from)(input)
}

/// Parses a WebAssembly memory component from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html#memory-section>
pub fn parse_memory(input: &[u8]) -> IResult<&[u8], Memory> {
    map(parse_memory_type, Memory::from)(input)
}

/// Parses a WebAssembly global component from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html#global-section>
pub fn parse_global(input: &[u8]) -> IResult<&[u8], Global> {
    map(
        tuple((parse_global_type, parse_expression)),
        |(kind, initializer)| Global::new(kind, initializer),
    )(input)
}
