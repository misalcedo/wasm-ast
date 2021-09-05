use crate::parser::types::parse_function_type;
use crate::parser::values::{parse_name, parse_u32, parse_vector};
use crate::{Custom, FunctionType, ModuleSection};
use nom::bytes::complete::{tag, take};
use nom::combinator::{all_consuming, map_parser};
use nom::IResult;

/// Parses a section with the given identifier. Validates the section identified and length.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html#sections>
fn parse_section(section: ModuleSection) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    move |input| {
        let (input, _) = tag(&[section as u8])(input)?;
        let (input, length) = parse_u32(input)?;

        take(length)(input)
    }
}

/// Parses a WebAssembly custom section.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html#binary-customsec>
pub fn parse_custom_section(input: &[u8]) -> IResult<&[u8], Custom> {
    let (remaining, section) = parse_section(ModuleSection::Custom)(input)?;
    let (contents, name) = parse_name(section)?;

    Ok((remaining, Custom::new(name, Vec::from(contents))))
}

/// Parses a WebAssembly type section.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html#binary-typesec>
pub fn parse_type_section(input: &[u8]) -> IResult<&[u8], Vec<FunctionType>> {
    map_parser(
        parse_section(ModuleSection::Type),
        all_consuming(parse_vector(parse_function_type)),
    )(input)
}
