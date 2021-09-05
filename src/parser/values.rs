use crate::Name;
use nom::bytes::complete::{take, take_while_m_n};
use nom::combinator::map_res;
use nom::IResult;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unsigned_leb128_large() {
        let input = vec![0xE5, 0x8E, 0x26];
        let (remaining, actual) = parse_u32(input.as_slice()).unwrap();

        assert_eq!(actual, 624485);
        assert_eq!(remaining, &[])
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
}
