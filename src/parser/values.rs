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

/// The high-order bit is equal to 0.
fn is_leb128_terminator(byte: u8) -> bool {
    byte & BASE == 0
}

/// The high-order bit is not equal to 0.
fn is_leb128_encoding(byte: u8) -> bool {
    byte & BASE != 0
}

/// Parses an unsigned 32-bit integer using LEB128 (Little-Endian Base 128) encoding.
pub fn parse_u32(input: &[u8]) -> IResult<&[u8], u32> {
    let (remaining, input) =
        take_while_m_n(1, max_leb128_size::<u32>(), is_leb128_encoding)(input)?;
    let (remaining, _) = take_while_m_n(1, 1, is_leb128_terminator)(remaining)?;

    let mut result = 0;
    for (index, byte) in input.iter().enumerate() {
        let part = (byte & !BASE) as u32;

        result |= part << (index * 7);
    }

    Ok((remaining, result))
}

pub fn parse_name(input: &[u8]) -> IResult<&[u8], Name> {
    let (input, length) = parse_u32(input)?;
    let (input, name) = map_res(take(length as usize), std::str::from_utf8)(input)?;

    Ok((input, name.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unsigned_leb128() {
        let input = vec![0xFF, 0x00, 0x11];
        let (remaining, actual) = parse_u32(input.as_slice()).unwrap();
        let expected = leb128::read::unsigned(&mut input.as_slice()).unwrap();

        assert_eq!(actual as u64, expected);
        assert_eq!(remaining, &[0x11u8])
    }
}
