use crate::Name;
use nom::bytes::complete::{tag, take, take_while_m_n};
use nom::combinator::{map, map_res};
use nom::multi::fold_many_m_n;
use nom::{IResult, Parser};
use std::convert::TryFrom;
use std::mem::size_of;

/// The radix (i.e. base) for LEB128 encoding.
const RADIX: u8 = 128;

/// Maximum size of a LEB128-encoded integer type
///
/// See <https://en.wikipedia.org/wiki/LEB128>
const fn max_leb128_size<T>() -> usize {
    let bits = size_of::<T>() * 8;

    (bits / 7) + (bits % 7 != 0) as usize
}

/// Parses a single byte and verified the parsed byte matches the given byte.
pub fn match_byte<'input>(byte: u8) -> impl FnMut(&'input [u8]) -> IResult<&'input [u8], u8> {
    map(tag([byte]), |bytes: &'input [u8]| bytes[0])
}

/// Parses an unsigned 32-bit integer using LEB128 (Little-Endian Base 128) encoding.
///
/// See <https://webassembly.github.io/spec/core/binary/values.html#integers>
pub fn parse_u32(input: &[u8]) -> IResult<&[u8], u32> {
    let (remaining, input) =
        take_while_m_n(0, max_leb128_size::<u32>() - 1, |x| x & RADIX != 0)(input)?;
    let (remaining, last) = take_while_m_n(1, 1, |x| x & RADIX == 0)(remaining)?;
    let mut result = 0;

    for (index, byte) in input.iter().chain(last.iter()).enumerate() {
        let part = (byte & !RADIX) as u32;

        result |= part << (index * 7);
    }

    Ok((remaining, result))
}

/// Parses a signed 33-bit integer using LEB128 (Little-Endian Base 128) encoding.
///
/// See <https://webassembly.github.io/spec/core/binary/values.html#integers>
pub fn parse_s33(input: &[u8]) -> IResult<&[u8], u32> {
    let (remaining, input) = take_while_m_n(0, 4, |x| x & RADIX != 0)(input)?;
    let (remaining, last) = take_while_m_n(1, 1, |x| x & RADIX == 0)(remaining)?;
    let mut result = 0;

    for (index, byte) in input.iter().chain(last.iter()).enumerate() {
        let part = (byte & !RADIX) as i64;

        result |= part << (index * 7);
    }

    if let Some(byte) = last.iter().next() {
        if byte & 0x40 == 0x40 {
            result |= !0 << ((input.len() + last.len()) * 7);
        }
    }

    map_res(move |i| Ok((i, result)), u32::try_from)(remaining)
}

/// Parses a signed 32-bit integer using LEB128 (Little-Endian Base 128) encoding.
///
/// See <https://webassembly.github.io/spec/core/binary/values.html#integers>
pub fn parse_s32(input: &[u8]) -> IResult<&[u8], i32> {
    Ok((input, 0))
}

/// Parses a signed 32-bit integer using LEB128 (Little-Endian Base 128) encoding.
///
/// See <https://webassembly.github.io/spec/core/binary/values.html#integers>
pub fn parse_s64(input: &[u8]) -> IResult<&[u8], i64> {
    Ok((input, 0))
}

/// Parses a WebAssembly name value.
///
/// See <https://webassembly.github.io/spec/core/binary/values.html#names>
pub fn parse_name(input: &[u8]) -> IResult<&[u8], Name> {
    map(map_res(parse_byte_vector, std::str::from_utf8), Name::from)(input)
}

/// Parses a WebAssembly byte vector.
///
/// See <https://webassembly.github.io/spec/core/binary/values.html#bytes>
pub fn parse_byte_vector(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, length) = parse_u32(input)?;
    let (input, bytes) = take(length as usize)(input)?;

    Ok((input, bytes))
}

/// Parses a WebAssembly encoded vector of items from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/conventions.html#vectors>
pub fn parse_vector<'input, O, P>(
    parser: P,
) -> impl Fn(&'input [u8]) -> IResult<&'input [u8], Vec<O>>
where
    P: Copy + Parser<&'input [u8], O, nom::error::Error<&'input [u8]>>,
{
    move |input| {
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

        let (remaining, parsed_vector): (&[u8], Vec<u8>) =
            parse_vector(take_byte)(input.as_slice()).unwrap();
        let vector_name = Name::new(String::from_utf8(parsed_vector).unwrap());

        assert_eq!(vector_name, Name::from(name));
        assert_eq!(remaining, &[extra]);
    }

    #[test]
    fn match_byte_matching() {
        let extra = 3;
        let byte = 42;
        let input = vec![byte, extra];

        let (remaining, actual): (&[u8], u8) = match_byte(byte)(input.as_slice()).unwrap();

        assert_eq!(actual, byte);
        assert_eq!(remaining, &[extra]);
    }

    #[test]
    fn match_byte_not_matching() {
        let input = vec![3];

        let result = match_byte(42)(input.as_slice());

        assert!(result.is_err());
    }

    fn take_byte(input: &[u8]) -> IResult<&[u8], u8> {
        map(take(1usize), |x: &[u8]| x[0])(input)
    }
}
