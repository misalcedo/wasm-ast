use crate::parser::values::match_byte;
use crate::{ControlInstruction, Expression, Instruction};
use nom::combinator::map;
use nom::multi::fold_many0;
use nom::sequence::terminated;
use nom::IResult;

/// Parses a WebAssembly expression from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/instructions.html#expressions>
pub fn parse_expression(input: &[u8]) -> IResult<&[u8], Expression> {
    parse_expression_with_terminal(0x0B)(input)
}

/// Parses a WebAssembly expression with a given terminating opcode from the input.
fn parse_expression_with_terminal<'input>(
    terminal: u8,
) -> impl FnMut(&'input [u8]) -> IResult<&'input [u8], Expression> {
    map(
        terminated(
            fold_many0(parse_instruction, Vec::new, |mut accumulator, item| {
                accumulator.push(item);
                accumulator
            }),
            match_byte(terminal),
        ),
        Expression::from,
    )
}

/// Parses a WebAssembly instruction from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/instructions.html>
pub fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    Ok((input, ControlInstruction::Nop.into()))
}
