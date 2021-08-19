/// Number types classify numeric values.
/// Number types are transparent, meaning that their bit patterns can be observed.
/// Values of number type can be stored in memories.
///
/// See https://webassembly.github.io/spec/core/syntax/types.html#number-types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NumberType {
    Integer(IntegerType),
    Float(FloatType),
}

impl From<IntegerType> for NumberType {
    fn from(kind: IntegerType) -> Self {
        NumberType::Integer(kind)
    }
}

impl From<FloatType> for NumberType {
    fn from(kind: FloatType) -> Self {
        NumberType::Float(kind)
    }
}

/// The types 𝗂𝟥𝟤 and 𝗂𝟨𝟦 classify 32 and 64 bit integers, respectively.
/// Integers are not inherently signed or unsigned, their interpretation is determined by individual operations.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IntegerType {
    I32,
    I64,
}

/// The types 𝖿𝟥𝟤 and 𝖿𝟨𝟦 classify 32 and 64 bit floating-point data, respectively.
/// They correspond to the respective binary floating-point representations,
/// also known as single and double precision, as defined by the IEEE 754-2019 standard (Section 3.3).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FloatType {
    F32,
    F64,
}

/// Reference types classify first-class references to objects in the runtime store.
/// The type 𝖿𝗎𝗇𝖼𝗋𝖾𝖿 denotes the infinite union of all references to functions,
/// regardless of their function types.
/// The type 𝖾𝗑𝗍𝖾𝗋𝗇𝗋𝖾𝖿 denotes the infinite union of all references to objects owned by the
/// embedder and that can be passed into WebAssembly under this type.
/// Reference types are opaque, meaning that neither their size nor their bit pattern can be observed.
/// Values of reference type can be stored in tables.
///
/// See https://webassembly.github.io/spec/core/syntax/types.html#reference-types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReferenceType {
    Function,
    External,
}

/// Value types classify the individual values that WebAssembly code can compute with and the values that a variable accepts.
/// They are either number types or reference types.
///
/// See https://webassembly.github.io/spec/core/syntax/types.html#value-types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValueType {
    Number(NumberType),
    Reference(ReferenceType),
}

impl<T> From<T> for ValueType
where
    T: Into<NumberType>,
{
    fn from(kind: T) -> Self {
        ValueType::Number(kind.into())
    }
}

impl From<ReferenceType> for ValueType {
    fn from(kind: ReferenceType) -> Self {
        ValueType::Reference(kind)
    }
}

/// Result types classify the result of executing instructions or functions,
/// which is a sequence of values, written with brackets.
///
/// See https://webassembly.github.io/spec/core/syntax/types.html#result-types
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResultType {
    kinds: Vec<ValueType>,
}

impl ResultType {
    /// Creates a new emtpy `ResultType`.
    pub fn new() -> Self {
        ResultType { kinds: Vec::new() }
    }

    /// A reference to a slice of the `ValueType`s.
    pub fn kinds(&self) -> &[ValueType] {
        &self.kinds
    }

    /// The length of the `ValueType` vector.
    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    /// Returns true if this `ResultType` has a length of zero, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.kinds.is_empty()
    }
}

impl From<Vec<ValueType>> for ResultType {
    fn from(kinds: Vec<ValueType>) -> Self {
        ResultType { kinds }
    }
}

/// Function types classify the signature of functions,
/// mapping a vector of parameters to a vector of results.
/// They are also used to classify the inputs and outputs of instructions
///
/// See https://webassembly.github.io/spec/core/syntax/types.html#function-types
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionType {
    parameters: ResultType,
    results: ResultType,
}

impl FunctionType {
    /// Creates a new function signature with the given parameter and result types.
    pub fn new(parameters: ResultType, results: ResultType) -> Self {
        FunctionType {
            parameters,
            results,
        }
    }

    /// The parameter types of this `FunctionType`.
    pub fn parameters(&self) -> &ResultType {
        &self.parameters
    }

    /// The result types of this `FunctionType`.
    pub fn results(&self) -> &ResultType {
        &self.results
    }
}

/// Limits classify the size range of resizeable storage associated with memory types and table types.
///
/// See https://webassembly.github.io/spec/core/syntax/types.html#limits
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Limit {
    min: u32,
    max: Option<u32>,
}

impl Limit {
    /// Creates a new limit with a required minimum and optional maximum.
    pub fn new(min: u32, max: Option<u32>) -> Self {
        Limit { min, max }
    }

    /// The minimum value of the limit.
    pub fn min(&self) -> u32 {
        self.min
    }

    /// The optional maximum value of the limit.
    pub fn max(&self) -> Option<u32> {
        self.max
    }
}

impl From<u32> for Limit {
    fn from(min: u32) -> Self {
        Limit { min, max: None }
    }
}

/// Memory types classify linear memories and their size range.
/// The limits constrain the minimum and optionally the maximum size of a memory.
/// The limits are given in units of page size.
///
/// See https://webassembly.github.io/spec/core/syntax/types.html#memory-types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MemoryType {
    limits: Limit,
}

impl MemoryType {
    /// The limits of the number of pages for this `MemoryType`.
    pub fn limits(&self) -> &Limit {
        &self.limits
    }
}

impl From<Limit> for MemoryType {
    fn from(limit: Limit) -> Self {
        MemoryType { limits: limit }
    }
}

/// Table types classify tables over elements of reference type within a size range.
/// Like memories, tables are constrained by limits for their minimum and optionally maximum size.
/// The limits are given in numbers of entries.
///
/// See https://webassembly.github.io/spec/core/syntax/types.html#table-types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TableType {
    limits: Limit,
    kind: ReferenceType,
}

impl TableType {
    /// Creates a new `TableType` for the given limits and reference type.
    pub fn new(limits: Limit, kind: ReferenceType) -> Self {
        TableType { limits, kind }
    }

    /// The limits of the number of elements for this `TableType`.
    pub fn limits(&self) -> &Limit {
        &self.limits
    }

    /// The reference type of the elements of this `TableType`.
    pub fn kind(&self) -> &ReferenceType {
        &self.kind
    }
}

/// Global types classify global variables, which hold a value and can either be mutable or immutable.
///
/// See https://webassembly.github.io/spec/core/syntax/types.html#global-types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GlobalType {
    mutability: Mutability,
    kind: ValueType,
}

impl GlobalType {
    /// Creates a new `GlobalType` for a mutable global variable.
    pub fn mutable(kind: ValueType) -> Self {
        GlobalType {
            mutability: Mutability::Mutable,
            kind,
        }
    }

    /// Creates a new `GlobalType` for an immutable (i.e. contant) global variable.
    pub fn immutable(kind: ValueType) -> Self {
        GlobalType {
            mutability: Mutability::Immutable,
            kind,
        }
    }

    /// The `ValueType` of the global variable defined by this `GlobalType`.
    pub fn kind(&self) -> &ValueType {
        &self.kind
    }

    pub fn mutability(&self) -> Mutability {
        self.mutability
    }
}

/// The mutability of a global variable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mutability {
    Mutable,
    Immutable,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_types() {
        assert_eq!(
            NumberType::Integer(IntegerType::I32),
            IntegerType::I32.into()
        );
        assert_eq!(
            NumberType::Integer(IntegerType::I64),
            IntegerType::I64.into()
        );
        assert_eq!(NumberType::Float(FloatType::F32), FloatType::F32.into());
        assert_eq!(NumberType::Float(FloatType::F64), FloatType::F64.into());
    }

    #[test]
    fn value_types() {
        assert_eq!(
            ValueType::Number(NumberType::Integer(IntegerType::I32)),
            IntegerType::I32.into()
        );
        assert_eq!(
            ValueType::Number(NumberType::Integer(IntegerType::I32)),
            NumberType::Integer(IntegerType::I32).into()
        );
        assert_eq!(
            ValueType::Number(NumberType::Integer(IntegerType::I64)),
            IntegerType::I64.into()
        );
        assert_eq!(
            ValueType::Number(NumberType::Integer(IntegerType::I64)),
            NumberType::Integer(IntegerType::I64).into()
        );
        assert_eq!(
            ValueType::Number(NumberType::Float(FloatType::F32)),
            FloatType::F32.into()
        );
        assert_eq!(
            ValueType::Number(NumberType::Float(FloatType::F32)),
            NumberType::Float(FloatType::F32).into()
        );
        assert_eq!(
            ValueType::Number(NumberType::Float(FloatType::F64)),
            FloatType::F64.into()
        );
        assert_eq!(
            ValueType::Number(NumberType::Float(FloatType::F64)),
            NumberType::Float(FloatType::F64).into()
        );
        assert_eq!(
            ValueType::Reference(ReferenceType::Function),
            ReferenceType::Function.into()
        );
        assert_eq!(
            ValueType::Reference(ReferenceType::External),
            ReferenceType::External.into()
        );
    }

    #[test]
    fn new_function_type() {
        let function_type = FunctionType::new(ResultType::new(), ResultType::new());

        assert!(function_type.parameters().is_empty());
        assert!(function_type.results().is_empty());
    }

    #[test]
    fn new_result_type() {
        let result_type = ResultType::from(vec![
            IntegerType::I32.into(),
            IntegerType::I64.into(),
            FloatType::F32.into(),
            FloatType::F64.into(),
            ReferenceType::Function.into(),
            ReferenceType::External.into(),
        ]);

        assert_eq!(result_type.len(), 6);
        assert!(!result_type.is_empty());
        assert_eq!(
            result_type.kinds(),
            &[
                ValueType::Number(NumberType::Integer(IntegerType::I32)),
                ValueType::Number(NumberType::Integer(IntegerType::I64)),
                ValueType::Number(NumberType::Float(FloatType::F32)),
                ValueType::Number(NumberType::Float(FloatType::F64)),
                ValueType::Reference(ReferenceType::Function),
                ValueType::Reference(ReferenceType::External),
            ]
        );
    }

    #[test]
    fn limits() {
        let max = Some(2);
        let min = 0;
        let limit = Limit::new(min, max);

        assert_eq!(limit.min, min);
        assert_eq!(limit.max, max);

        assert_eq!(Limit::from(2), Limit::new(2, None));
    }

    #[test]
    fn memory_type() {
        let limit = Limit::from(0);
        let memory_type = MemoryType::from(limit.clone());

        assert_eq!(memory_type.limits(), &limit);
    }

    #[test]
    fn table_type() {
        let limit = Limit::new(0, None);
        let reference_type = ReferenceType::External;
        let table_type = TableType::new(limit.clone(), reference_type);

        assert_eq!(table_type.limits(), &limit);
        assert_eq!(table_type.kind(), &reference_type);
    }

    #[test]
    fn global_type() {
        let kind = ValueType::from(IntegerType::I64);
        let mutable = GlobalType::mutable(kind);

        assert_eq!(mutable.mutability(), Mutability::Mutable);
        assert_eq!(mutable.kind(), &kind);

        let immutable = GlobalType::immutable(kind);
        assert_eq!(immutable.mutability(), Mutability::Immutable);
        assert_eq!(immutable.kind(), &kind);
    }
}
