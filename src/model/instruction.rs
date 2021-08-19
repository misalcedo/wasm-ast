use crate::model::{
    DataIndex, ElementIndex, FloatType, FunctionIndex, GlobalIndex, IntegerType, LabelIndex,
    LocalIndex, NumberType, ReferenceType, TableIndex, TypeIndex, ValueType,
};
use std::mem::size_of;

/// WebAssembly code consists of sequences of instructions.
/// Its computational model is based on a stack machine in that instructions manipulate values on
/// an implicit operand stack, consuming (popping) argument values and producing or returning
/// (pushing) result values.
/// In addition to dynamic operands from the stack,
/// some instructions also have static immediate arguments,
/// typically indices or type annotations, which are part of the instruction itself.
/// Some instructions are structured in that they bracket nested sequences of instructions.
/// The following sections group instructions into a number of different categories.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#instructions
#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Numeric(NumericInstruction),
    Reference(ReferenceInstruction),
    Parametric(ParametricInstruction),
    Variable(VariableInstruction),
    Table(TableInstruction),
    Memory(MemoryInstruction),
    Control(ControlInstruction),
}

/// Numeric instructions provide basic operations over numeric values of specific type.
/// These operations closely match respective operations available in hardware.
///
/// Some integer instructions come in two flavors,
/// where a signedness annotation sx distinguishes whether the operands are to be interpreted as
/// unsigned or signed integers. For the other integer instructions, the use of two’s complement
/// for the signed interpretation means that they behave the same regardless of signedness.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#numeric-instructions
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NumericInstruction {
    I32Constant(i32),
    I64Constant(i64),
    F32Constant(f32),
    F64Constant(f64),
    CountLeadingZeros(IntegerType),  // clz
    CountTrailingZeros(IntegerType), // ctz
    CountOnes(IntegerType),          // popcnt
    AbsoluteValue(FloatType),
    Negate(FloatType),
    SquareRoot(FloatType),
    Ceiling(FloatType),
    Floor(FloatType),
    Truncate(FloatType),
    Nearest(FloatType),
    Add(NumberType),
    Subtract(NumberType),
    Multiply(NumberType),
    DivideInteger(IntegerType, SignExtension),
    DivideFloat(FloatType),
    Remainder(IntegerType, SignExtension),
    And(IntegerType),
    Or(IntegerType),
    Xor(IntegerType),
    ShiftLeft(IntegerType),
    ShiftRight(IntegerType, SignExtension),
    RotateLeft(IntegerType),
    RotateRight(IntegerType),
    Minimum(FloatType),
    Maximum(FloatType),
    CopySign(FloatType),
    EqualToZero(IntegerType),
    Equal(NumberType),
    NotEqual(NumberType),
    LessThanInteger(IntegerType, SignExtension),
    LessThanFloat(FloatType),
    GreaterThanInteger(IntegerType, SignExtension),
    GreaterThanFloat(FloatType),
    LessThanOrEqualToInteger(IntegerType, SignExtension),
    LessThanOrEqualToFloat(FloatType),
    GreaterThanOrEqualToInteger(IntegerType, SignExtension),
    GreaterThanOrEqualToFloat(FloatType),
    ExtendSigned8(IntegerType),
    ExtendSigned16(IntegerType),
    ExtendSigned32,
    Wrap,
    ExtendWithSignExtension(SignExtension),
    ConvertAndTruncate(IntegerType, FloatType, SignExtension), // trunc
    ConvertAndTruncateWithSaturation(IntegerType, FloatType, SignExtension), // trunc_sat
    Demote,
    Promote,
    Convert(FloatType, IntegerType, SignExtension),
    ReinterpretFloat(IntegerType, FloatType),
    ReinterpretInteger(FloatType, IntegerType),
}

/// Instructions in this group are concerned with accessing references.
/// These instruction produce a null value, check for a null value, or produce a reference to a given function, respectively.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#reference-instructions
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReferenceInstruction {
    /// Produce a null value.
    ReferenceNull(ReferenceType),
    /// Check for a null value.
    ReferenceIsNull,
    /// Produce a reference to a given function.
    ReferenceFunction(FunctionIndex),
}

/// Instructions in this group can operate on operands of any value type.
///
/// https://webassembly.github.io/spec/core/syntax/instructions.html#parametric-instructions
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParametricInstruction {
    /// The 𝖽𝗋𝗈𝗉 instruction simply throws away a single operand.
    Drop,
    /// The 𝗌𝖾𝗅𝖾𝖼𝗍 instruction selects one of its first two operands based on whether its third
    /// operand is zero or not. It may include a value type determining the type of these operands.
    /// If missing, the operands must be of numeric type.
    Select(Option<Vec<ValueType>>),
}

/// Variable instructions are concerned with access to local or global variables.
/// These instructions get or set the values of variables, respectively.
/// The 𝗅𝗈𝖼𝖺𝗅.𝗍𝖾𝖾 instruction is like 𝗅𝗈𝖼𝖺𝗅.𝗌𝖾𝗍 but also returns its argument.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#variable-instructions
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VariableInstruction {
    /// Get the value of a local variable.
    LocalGet(LocalIndex),
    /// Set the value of a local variable.
    LocalSet(LocalIndex),
    /// The 𝗅𝗈𝖼𝖺𝗅.𝗍𝖾𝖾 instruction is like 𝗅𝗈𝖼𝖺𝗅.𝗌𝖾𝗍 but also returns its argument.
    LocalTee(LocalIndex),
    /// Get the value of a global variable.
    GlobalGet(GlobalIndex),
    /// Set the value of a global variable.
    GlobalSet(GlobalIndex),
}

/// Instructions in this group are concerned with tables table.
/// An additional instruction that accesses a table is the control instruction 𝖼𝖺𝗅𝗅_𝗂𝗇𝖽𝗂𝗋𝖾𝖼𝗍.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#table-instructions
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TableInstruction {
    /// The 𝗍𝖺𝖻𝗅𝖾.𝗀𝖾𝗍 instruction loads an element in a table.
    TableGet(TableIndex),
    /// The 𝗍𝖺𝖻𝗅𝖾.𝗌𝖾𝗍 instruction stores an element in a table.
    TableSet(TableIndex),
    /// The 𝗍𝖺𝖻𝗅𝖾.𝗌𝗂𝗓𝖾 instruction returns the current size of a table.
    TableSize(TableIndex),
    /// The 𝗍𝖺𝖻𝗅𝖾.𝗀𝗋𝗈𝗐 instruction grows table by a given delta and returns the previous size,
    /// or −1 if enough space cannot be allocated.
    /// It also takes an initialization value for the newly allocated entries.
    TableGrow(TableIndex),
    /// The 𝗍𝖺𝖻𝗅𝖾.𝖿𝗂𝗅𝗅 instruction sets all entries in a range to a given value.
    TableFill(TableIndex),
    /// The 𝗍𝖺𝖻𝗅𝖾.𝖼𝗈𝗉𝗒 instruction copies elements from a source table region to a
    /// possibly overlapping destination region; the first index denotes the destination.
    TableCopy(TableIndex, TableIndex),
    /// The 𝗍𝖺𝖻𝗅𝖾.𝗂𝗇𝗂𝗍 instruction copies elements from a passive element segment into a table.
    TableInit(ElementIndex, TableIndex),
    /// The 𝖾𝗅𝖾𝗆.𝖽𝗋𝗈𝗉 instruction prevents further use of a passive element segment.
    /// This instruction is intended to be used as an optimization hint.
    /// After an element segment is dropped its elements can no longer be retrieved,
    /// so the memory used by this segment may be freed.
    ElementDrop(ElementIndex),
}

/// Instructions in this group are concerned with linear memory.
/// Memory is accessed with 𝗅𝗈𝖺𝖽 and 𝗌𝗍𝗈𝗋𝖾 instructions for the different value types.
/// They all take a memory immediate memarg that contains an address offset and
/// the expected alignment (expressed as the exponent of a power of 2).
/// Integer loads and stores can optionally specify a storage size that is smaller than
/// the bit width of the respective value type.
/// In the case of loads, a sign extension mode sx is then required to select appropriate behavior.
///
/// The static address offset is added to the dynamic address operand,
/// yielding a 33 bit effective address that is the zero-based index at which the memory is accessed.
/// All values are read and written in little endian byte order.
/// A trap results if any of the accessed memory bytes lies outside the address range implied by
/// the memory’s current size.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#memory-instructions
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MemoryInstruction {
    /// Load a number type from memory.
    Load(NumberType, MemoryArgument),
    /// Store a number type from memory.
    Store(NumberType, MemoryArgument),
    /// Integer load that specifies a storage size that is smaller than
    /// the bit width of the respective value type.
    Load8(IntegerType, SignExtension, MemoryArgument),
    Load16(IntegerType, SignExtension, MemoryArgument),
    Load32(SignExtension, MemoryArgument),
    /// Integer store that specifies a storage size that is smaller than
    /// the bit width of the respective value type.
    Store8(IntegerType, MemoryArgument),
    Store16(IntegerType, MemoryArgument),
    Store32(MemoryArgument),
    /// The 𝗆𝖾𝗆𝗈𝗋𝗒.𝗌𝗂𝗓𝖾 instruction returns the current size of a memory.
    /// Operates in units of page size.
    MemorySize,
    /// The 𝗆𝖾𝗆𝗈𝗋𝗒.𝗀𝗋𝗈𝗐 instruction grows memory by a given delta and returns the previous size,
    /// or −1 if enough memory cannot be allocated.
    MemoryGrow,
    /// The 𝗆𝖾𝗆𝗈𝗋𝗒.𝖿𝗂𝗅𝗅 instruction sets all values in a region to a given byte.
    MemoryFill,
    /// The 𝗆𝖾𝗆𝗈𝗋𝗒.𝖼𝗈𝗉𝗒 instruction copies data from a source memory region to
    /// a possibly overlapping destination region.
    MemoryCopy,
    /// The 𝗆𝖾𝗆𝗈𝗋𝗒.𝗂𝗇𝗂𝗍 instruction copies data from a passive data segment into a memory.
    MemoryInit(DataIndex),
    /// he 𝖽𝖺𝗍𝖺.𝖽𝗋𝗈𝗉 instruction prevents further use of a passive data segment.
    /// This instruction is intended to be used as an optimization hint.
    /// After a data segment is dropped its data can no longer be retrieved,
    /// so the memory used by this segment may be freed.
    DataDrop(DataIndex),
}

/// Instructions in this group affect the flow of control.
/// The 𝖻𝗅𝗈𝖼𝗄, 𝗅𝗈𝗈𝗉 and 𝗂𝖿 instructions are structured instructions.
/// They bracket nested sequences of instructions, called blocks, terminated with, or separated by,
/// 𝖾𝗇𝖽 or 𝖾𝗅𝗌𝖾 pseudo-instructions. As the grammar prescribes, they must be well-nested.
///
/// A structured instruction can consume input and produce output on the operand stack according to
/// its annotated block type. It is given either as a type index that refers to a suitable function
/// type, or as an optional value type inline,
/// which is a shorthand for the function type []→[valtype?].
///
/// Each structured control instruction introduces an implicit label.
/// Labels are targets for branch instructions that reference them with label indices.
/// Unlike with other index spaces, indexing of labels is relative by nesting depth, that is,
/// label 0 refers to the innermost structured control instruction enclosing the referring branch
/// instruction, while increasing indices refer to those farther out.
/// Consequently, labels can only be referenced from within the associated structured control
/// instruction. This also implies that branches can only be directed outwards,
/// “breaking” from the block of the control construct they target.
/// The exact effect depends on that control construct.
/// In case of 𝖻𝗅𝗈𝖼𝗄 or 𝗂𝖿 it is a forward jump, resuming execution after the matching 𝖾𝗇𝖽.
/// In case of 𝗅𝗈𝗈𝗉 it is a backward jump to the beginning of the loop.
///
/// Taking a branch unwinds the operand stack up to the height where the targeted structured
/// control instruction was entered. However, branches may additionally consume operands themselves,
/// which they push back on the operand stack after unwinding.
/// Forward branches require operands according to the output of the targeted block’s type, i.e.,
/// represent the values produced by the terminated block.
/// Backward branches require operands according to the input of the targeted block’s type, i.e.,
/// represent the values consumed by the restarted block.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#control-instructions
#[derive(Clone, Debug, PartialEq)]
pub enum ControlInstruction {
    /// The 𝗇𝗈𝗉 instruction does nothing.
    Nop,
    /// The 𝗎𝗇𝗋𝖾𝖺𝖼𝗁𝖺𝖻𝗅𝖾 instruction causes an unconditional trap.
    Unreachable,
    /// A logical grouping used introduce a label around an expression.
    Block(BlockType, Expression),
    /// Executes the expression in a loop.
    Loop(BlockType, Expression),
    /// Conditionally executes a positive or (optional) negative branch based on a test value.
    If(BlockType, Expression, Option<Expression>),
    /// The 𝖻𝗋 instruction performs an unconditional branch.
    Branch(LabelIndex),
    /// The 𝖻𝗋_𝗂𝖿 instruction performs a conditional branch
    BranchIf(LabelIndex),
    /// The 𝖻𝗋_𝗍𝖺𝖻𝗅𝖾 instruction performs an indirect branch through an operand indexing into
    /// the label vector that is an immediate to the instruction,
    /// or to a default target if the operand is out of bounds.
    BranchTable(Vec<LabelIndex>, LabelIndex),
    /// he 𝗋𝖾𝗍𝗎𝗋𝗇 instruction is a shortcut for an unconditional branch to the outermost block,
    /// which implicitly is the body of the current function.
    Return,
    /// The 𝖼𝖺𝗅𝗅 instruction invokes another function, consuming the necessary arguments from
    /// the stack and returning the result values of the call.
    Call(FunctionIndex),
    /// The 𝖼𝖺𝗅𝗅_𝗂𝗇𝖽𝗂𝗋𝖾𝖼𝗍 instruction calls a function indirectly through an operand indexing into
    /// a table that is denoted by a table index and must have type 𝖿𝗎𝗇𝖼𝗋𝖾𝖿.
    /// Since it may contain functions of heterogeneous type,
    /// the callee is dynamically checked against the function type indexed by the instruction’s
    /// second immediate, and the call is aborted with a trap if it does not match.
    CallIndirect(TypeIndex, TableIndex),
}

/// A structured instruction can consume input and produce output on the operand stack according to
/// its annotated block type.
/// It is given either as a type index that refers to a suitable function type,
/// or as an optional value type inline, which is a shorthand for the function type []→[valtype?].
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#control-instructions
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlockType {
    None,
    Index(TypeIndex),
    ValueType(ValueType),
}

/// Argument to load and store instructions that contains an address offset and
/// the expected alignment (expressed as the exponent of a power of 2).
///
/// The static address offset is added to the dynamic address operand,
/// yielding a 33 bit effective address that is the zero-based index at which the memory is accessed.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#memory-instructions
///
/// # Examples
/// ## With Offset & Alignment
/// ```rust
/// use wasm_ast::MemoryArgument;
///
/// let argument = MemoryArgument::new(42, 3);
///
/// assert_eq!(argument.offset(), 42);
/// assert_eq!(argument.align(), 3);
/// ```
///
/// ## With Offset Only
/// ```rust
/// use wasm_ast::MemoryArgument;
///
/// let argument = MemoryArgument::offset_default::<u8>(42);
///
/// assert_eq!(argument.offset(), 42);
/// assert_eq!(argument.align(), 1);
/// ```
///
/// ## With Alignment Only
/// ```rust
/// use wasm_ast::MemoryArgument;
///
/// let argument = MemoryArgument::aligned(4);
///
/// assert_eq!(argument.offset(), 0);
/// assert_eq!(argument.align(), 4);
/// ```
///
/// ## Default
/// ```rust
/// use wasm_ast::MemoryArgument;
///
/// let argument = MemoryArgument::default::<u16>();
///
/// assert_eq!(argument.offset(), 0);
/// assert_eq!(argument.align(), 2);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MemoryArgument {
    offset: u32,
    align: u32,
}

impl MemoryArgument {
    /// Creates a new memory argument with the given offset and alignment.
    pub fn new(offset: u32, align: u32) -> Self {
        MemoryArgument { offset, align }
    }

    /// Creates a new memory argument with the given alignment and an offset of 0.
    pub fn aligned(align: u32) -> Self {
        MemoryArgument { offset: 0, align }
    }

    /// Creates a new memory argument with the default alignment and an offset of 0.
    pub fn default<T>() -> Self {
        MemoryArgument {
            offset: 0,
            align: size_of::<T>() as u32,
        }
    }

    /// Creates a new memory argument with the default alignment and the given offset.
    pub fn offset_default<T>(offset: u32) -> Self {
        MemoryArgument {
            offset,
            align: size_of::<T>() as u32,
        }
    }

    /// The static address offset of the memory instruction.
    pub fn offset(&self) -> u32 {
        self.offset
    }

    /// The memory alignment of the instruction expressed as the exponent of a power of 2.
    pub fn align(&self) -> u32 {
        self.align
    }
}

/// Some integer instructions come in two flavors, where a signedness annotation sx distinguishes
/// whether the operands are to be interpreted as unsigned or signed integers.
/// For the other integer instructions, the use of two’s complement for the signed interpretation
/// means that they behave the same regardless of signedness.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#numeric-instructions
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SignExtension {
    Signed,
    Unsigned,
}

/// Function bodies, initialization values for globals,
/// and offsets of element or data segments are given as expressions, which are sequences of instructions terminated by an 𝖾𝗇𝖽 marker.
/// In some places, validation restricts expressions to be constant,
/// which limits the set of allowable instructions.
///
/// See https://webassembly.github.io/spec/core/syntax/instructions.html#expressions
#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    instructions: Vec<Instruction>,
}

impl Expression {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Expression { instructions }
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_expression() {
        let instruction = Instruction::Control(ControlInstruction::Nop);
        let expression = Expression::new(vec![instruction.clone()]);

        assert_eq!(expression.instructions(), &[instruction]);
        assert!(!expression.is_empty());
    }

    #[test]
    fn new_memory_argument() {
        let align = 0;
        let offset = 42;
        let argument = MemoryArgument::new(align, offset);

        assert_eq!(argument.align(), align);
        assert_eq!(argument.offset(), offset);
    }
}
