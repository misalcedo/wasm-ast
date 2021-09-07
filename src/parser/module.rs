use crate::parser::types::{parse_global_type, parse_memory_type, parse_table_type};
use crate::parser::values::{match_byte, parse_name, parse_u32};
use crate::{Import, ImportDescription};
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
