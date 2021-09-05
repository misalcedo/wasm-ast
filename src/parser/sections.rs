use crate::parser::values::{parse_name, parse_u32};
use crate::{Custom, ModuleSection};
use nom::bytes::complete::{tag, take};
use nom::IResult;

/// Parses a section with the given identifier. Validates the section identified and length.
fn parse_section(section: ModuleSection) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    move |input| {
        let (input, _) = tag(&[section as u8])(input)?;
        let (input, length) = parse_u32(input)?;

        take(length)(input)
    }
}

/// Parses a custom section.
pub fn parse_custom_section(input: &[u8]) -> IResult<&[u8], Custom> {
    let (remaining, section) = parse_section(ModuleSection::Custom)(input)?;
    let (contents, name) = parse_name(section)?;

    Ok((remaining, Custom::new(name, Vec::from(contents))))
}
