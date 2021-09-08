use crate::parser::types::{parse_reference_type, parse_value_type};
use crate::parser::values::{match_byte, parse_u32, parse_vector};
use crate::{
    ControlInstruction, Expression, Instruction, MemoryInstruction, NumericInstruction,
    ParametricInstruction, ReferenceInstruction, TableInstruction, VariableInstruction,
};
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::fold_many0;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

/// Marks the end of an expression.
const EXPRESSION_END: u8 = 0x0B;

/// Parses a WebAssembly expression from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/instructions.html#expressions>
pub fn parse_expression(input: &[u8]) -> IResult<&[u8], Expression> {
    parse_expression_with_terminal(EXPRESSION_END)(input)
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
        Expression::new,
    )
}

/// Parses a WebAssembly instruction from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/instructions.html>
pub fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    alt((
        map(parse_control_instruction, Instruction::from),
        map(parse_reference_instruction, Instruction::from),
        map(parse_parametric_instruction, Instruction::from),
        map(parse_variable_instruction, Instruction::from),
        map(parse_table_instruction, Instruction::from),
        map(parse_memory_instruction, Instruction::from),
        map(parse_numeric_instruction, Instruction::from),
    ))(input)
}

/// Parses a WebAssembly control instruction from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/instructions.html#control-instructions>
pub fn parse_control_instruction(input: &[u8]) -> IResult<&[u8], ControlInstruction> {
    Ok((&input[1..], ControlInstruction::Nop))
}

/// Parses a WebAssembly reference instruction from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/instructions.html#reference-instructions>
pub fn parse_reference_instruction(input: &[u8]) -> IResult<&[u8], ReferenceInstruction> {
    alt((
        map(
            preceded(match_byte(0xD0), parse_reference_type),
            ReferenceInstruction::Null,
        ),
        map(match_byte(0xD1), |_| ReferenceInstruction::IsNull),
        map(
            preceded(match_byte(0xD2), parse_u32),
            ReferenceInstruction::Function,
        ),
    ))(input)
}

/// Parses a WebAssembly parametric instruction from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/instructions.html#parametric-instructions>
pub fn parse_parametric_instruction(input: &[u8]) -> IResult<&[u8], ParametricInstruction> {
    alt((
        map(match_byte(0x1A), |_| ParametricInstruction::Drop),
        map(match_byte(0x1B), |_| ParametricInstruction::Select(None)),
        map(
            preceded(match_byte(0x1C), parse_vector(parse_value_type)),
            |value_types| ParametricInstruction::Select(Some(value_types)),
        ),
    ))(input)
}

/// Parses a WebAssembly variable instruction from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/instructions.html#variable-instructions>
pub fn parse_variable_instruction(input: &[u8]) -> IResult<&[u8], VariableInstruction> {
    alt((
        map(
            preceded(match_byte(0x20), parse_u32),
            VariableInstruction::LocalGet,
        ),
        map(
            preceded(match_byte(0x21), parse_u32),
            VariableInstruction::LocalSet,
        ),
        map(
            preceded(match_byte(0x22), parse_u32),
            VariableInstruction::LocalTee,
        ),
        map(
            preceded(match_byte(0x23), parse_u32),
            VariableInstruction::GlobalGet,
        ),
        map(
            preceded(match_byte(0x24), parse_u32),
            VariableInstruction::GlobalSet,
        ),
    ))(input)
}

/// Parses a WebAssembly table instruction from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/instructions.html#table-instructions>
pub fn parse_table_instruction(input: &[u8]) -> IResult<&[u8], TableInstruction> {
    alt((
        map(preceded(match_byte(0x25), parse_u32), TableInstruction::Get),
        map(preceded(match_byte(0x26), parse_u32), TableInstruction::Set),
        map(
            preceded(
                tuple((match_byte(0xFC), match_byte(12u8))),
                tuple((parse_u32, parse_u32)),
            ),
            |(element, table)| TableInstruction::Init(element, table),
        ),
        map(
            preceded(tuple((match_byte(0xFC), match_byte(13u8))), parse_u32),
            TableInstruction::ElementDrop,
        ),
        map(
            preceded(
                tuple((match_byte(0xFC), match_byte(14u8))),
                tuple((parse_u32, parse_u32)),
            ),
            |(table_a, table_b)| TableInstruction::Copy(table_a, table_b),
        ),
        map(
            preceded(tuple((match_byte(0xFC), match_byte(15u8))), parse_u32),
            TableInstruction::Grow,
        ),
        map(
            preceded(tuple((match_byte(0xFC), match_byte(16u8))), parse_u32),
            TableInstruction::Size,
        ),
        map(
            preceded(tuple((match_byte(0xFC), match_byte(17u8))), parse_u32),
            TableInstruction::Fill,
        ),
    ))(input)
}

/// Parses a WebAssembly memory instruction from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/instructions.html#memory-instructions>
pub fn parse_memory_instruction(input: &[u8]) -> IResult<&[u8], MemoryInstruction> {
    Ok((&input[1..], MemoryInstruction::Fill))
}

/// Parses a WebAssembly numeric instruction from the input.
///
/// See <https://webassembly.github.io/spec/core/binary/instructions.html#numeric-instructions>
pub fn parse_numeric_instruction(input: &[u8]) -> IResult<&[u8], NumericInstruction> {
    Ok((&input[1..], NumericInstruction::Demote))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_expression() {
        let extra = 3;
        let input = vec![EXPRESSION_END, extra];

        let (remaining, actual) = parse_expression(input.as_slice()).unwrap();

        assert_eq!(actual, Expression::empty());
        assert_eq!(remaining, &[extra]);
    }

    #[test]
    fn parse_expression_with_instruction() {
        let extra = 3;
        let byte = 0x01;
        let input = vec![byte, EXPRESSION_END, extra];

        let (remaining, actual) = parse_expression(input.as_slice()).unwrap();
        let expected = Expression::new(vec![ControlInstruction::Nop.into()]);

        assert_eq!(actual, expected);
        assert_eq!(remaining, &[extra]);
    }

    #[test]
    fn parse_invalid_expression() {
        let input = vec![3];
        let result = parse_expression(input.as_slice());

        assert!(result.is_err());
    }
}
