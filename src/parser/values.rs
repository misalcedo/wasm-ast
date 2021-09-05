use crate::Name;
use nom::bytes::complete::{take, take_while_m_n};
use nom::combinator::map_res;
use nom::multi::fold_many_m_n;
use nom::{IResult, Parser};
use std::mem::size_of;

const BASE: u8 = 128;

/// Maximum size of a n LEB128-encoded integer type
const fn max_leb128_size<T>() -> usize {
    let bits = size_of::<T>() * 8;

    (bits / 7) + (bits % 7 != 0) as usize
}

/// Parses an unsigned 32-bit integer using LEB128 (Little-Endian Base 128) encoding.
pub fn parse_u32(input: &[u8]) -> IResult<&[u8], u32> {
    let (remaining, input) =
        take_while_m_n(0, max_leb128_size::<u32>() - 1, |x| x & BASE != 0)(input)?;
    let (remaining, last) = take_while_m_n(1, 1, |x| x & BASE == 0)(remaining)?;
    let mut result = 0;

    for (index, byte) in input.iter().chain(last.iter()).enumerate() {
        let part = (byte & !BASE) as u32;

        result |= part << (index * 7);
    }

    Ok((remaining, result))
}

/// Parses a WebAssembly name value.
pub fn parse_name(input: &[u8]) -> IResult<&[u8], Name> {
    let (input, length) = parse_u32(input)?;
    let (input, name) = map_res(take(length as usize), std::str::from_utf8)(input)?;

    Ok((input, name.into()))
}

pub fn parse_vector<'input, O, P>(parser: P, input: &'input [u8]) -> IResult<&'input [u8], Vec<O>>
where
    P: Parser<&'input [u8], O, nom::error::Error<&'input [u8]>>,
{
    let (input, length) = parse_u32(input)?;
    let length = length as usize;
    let (remaining, items) = fold_many_m_n(
        length,
        length,
        parser,
        move || Vec::with_capacity(length),
        |mut accumulator, item| {
            accumulator.push(item);
            accumulator
        },
    )(input)?;

    Ok((remaining, items))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::take;
    use nom::combinator::map;

    #[test]
    fn parse_unsigned_leb128_large() {
        let input = vec![0xE5, 0x8E, 0x26];
        let (remaining, actual) = parse_u32(input.as_slice()).unwrap();

        assert_eq!(actual, 624485);
        assert!(remaining.is_empty())
    }

    #[test]
    fn parse_unsigned_leb128_small() {
        let input = vec![64, 0xFF];
        let (remaining, actual) = parse_u32(input.as_slice()).unwrap();

        assert_eq!(actual, 64);
        assert_eq!(remaining, &[0xFF])
    }

    #[test]
    fn parse_unsigned_leb128_zero() {
        let input = vec![0x00, 0xFF];
        let (remaining, actual) = parse_u32(input.as_slice()).unwrap();

        assert_eq!(actual, 0);
        assert_eq!(remaining, &[0xFF])
    }

    #[test]
    fn parse_name_with_remaining() {
        let name = "Hello, World!";
        let extra = 42;
        let mut input = Vec::from(name);
        input.insert(0, name.len() as u8);
        input.push(extra);

        let (remaining, parsed_name) = parse_name(input.as_slice()).unwrap();

        assert_eq!(parsed_name, Name::from(name));
        assert_eq!(remaining, &[extra]);
    }

    #[test]
    fn parse_vector_with_remaining() {
        let name = "Hello, World!";
        let extra = 42;
        let mut input = Vec::from(name);
        input.insert(0, name.len() as u8);
        input.push(extra);

        let take_byte = map(take(1usize), |x: &[u8]| x[0]);
        let (remaining, parsed_vector): (&[u8], Vec<u8>) =
            parse_vector(take_byte, input.as_slice()).unwrap();
        let vector_name = Name::new(String::from_utf8(parsed_vector).unwrap());

        assert_eq!(vector_name, Name::from(name));
        assert_eq!(remaining, &[extra]);
    }
}
