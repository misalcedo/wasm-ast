use serde::{Deserialize, Serialize};

/// Number types classify numeric values.
/// Number types are transparent, meaning that their bit patterns can be observed.
/// Values of number type can be stored in memories.
/// See https://webassembly.github.io/spec/core/syntax/types.html#number-types
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum NumberType {
    I32,
    I64,
    F32,
    F64,
}

/// The types 𝗂𝟥𝟤 and 𝗂𝟨𝟦 classify 32 and 64 bit integers, respectively.
/// Integers are not inherently signed or unsigned, their interpretation is determined by individual operations.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum IntegerType {
    I32,
    I64,
}

/// The types 𝖿𝟥𝟤 and 𝖿𝟨𝟦 classify 32 and 64 bit floating-point data, respectively.
/// They correspond to the respective binary floating-point representations,
/// also known as single and double precision, as defined by the IEEE 754-2019 standard (Section 3.3).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
/// See https://webassembly.github.io/spec/core/syntax/types.html#reference-types
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReferenceType {
    Function, // funcref
    External, // externref
}

/// Value types classify the individual values that WebAssembly code can compute with and the values that a variable accepts.
/// They are either number types or reference types.
/// See https://webassembly.github.io/spec/core/syntax/types.html#value-types
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ValueType {
    Number(NumberType),
    Reference(ReferenceType),
}

/// Result types classify the result of executing instructions or functions,
/// which is a sequence of values, written with brackets.
/// See https://webassembly.github.io/spec/core/syntax/types.html#result-types
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResultType {
    kinds: Vec<ValueType>,
}

impl ResultType {
    pub fn new(kinds: Vec<ValueType>) -> Self {
        ResultType { kinds }
    }

    pub fn kinds(&self) -> &[ValueType] {
        &self.kinds
    }

    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    pub fn is_empty(&self) -> bool {
        self.kinds.is_empty()
    }
}

/// Function types classify the signature of functions,
/// mapping a vector of parameters to a vector of results.
/// They are also used to classify the inputs and outputs of instructions.
/// See https://webassembly.github.io/spec/core/syntax/types.html#function-types
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctionType {
    parameters: ResultType,
    results: ResultType,
}

impl FunctionType {
    pub fn new(parameters: ResultType, results: ResultType) -> Self {
        FunctionType {
            parameters,
            results,
        }
    }

    pub fn parameters(&self) -> &ResultType {
        &self.parameters
    }

    pub fn results(&self) -> &ResultType {
        &self.results
    }
}

/// Limits classify the size range of resizeable storage associated with memory types and table types.
/// See https://webassembly.github.io/spec/core/syntax/types.html#limits
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Limit {
    min: usize,
    max: Option<usize>,
}

impl Limit {
    pub fn new(min: usize, max: Option<usize>) -> Self {
        Limit { min, max }
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn max(&self) -> Option<usize> {
        self.max
    }
}

/// Memory types classify linear memories and their size range.
/// The limits constrain the minimum and optionally the maximum size of a memory.
/// The limits are given in units of page size.
/// See https://webassembly.github.io/spec/core/syntax/types.html#memory-types
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MemoryType {
    limits: Limit,
}

impl MemoryType {
    pub fn new(limits: Limit) -> Self {
        MemoryType { limits }
    }

    pub fn limits(&self) -> &Limit {
        &self.limits
    }
}

/// Table types classify tables over elements of reference type within a size range.
/// Like memories, tables are constrained by limits for their minimum and optionally maximum size.
/// The limits are given in numbers of entries.
/// See https://webassembly.github.io/spec/core/syntax/types.html#table-types
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableType {
    limits: Limit,
    kind: ReferenceType,
}

impl TableType {
    pub fn new(limits: Limit, reference_type: ReferenceType) -> Self {
        TableType {
            limits,
            kind: reference_type,
        }
    }

    pub fn limits(&self) -> &Limit {
        &self.limits
    }

    pub fn kind(&self) -> &ReferenceType {
        &self.kind
    }
}

/// Global types classify global variables, which hold a value and can either be mutable or immutable.
/// See https://webassembly.github.io/spec/core/syntax/types.html#global-types
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GlobalType {
    is_mutable: bool,
    kind: ValueType,
}

impl GlobalType {
    pub fn new(is_mutable: bool, kind: ValueType) -> Self {
        GlobalType { is_mutable, kind }
    }

    pub fn kind(&self) -> &ValueType {
        &self.kind
    }

    pub fn is_mutable(&self) -> bool {
        self.is_mutable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_function_type() {
        let result_type = ResultType::new(Vec::new());
        let function_type = FunctionType::new(result_type.clone(), result_type.clone());

        assert!(function_type.parameters().is_empty());
        assert!(function_type.results().is_empty());
    }

    #[test]
    fn new_result_type() {
        let result_type = ResultType::new(vec![
            ValueType::Number(NumberType::I64),
            ValueType::Number(NumberType::F64),
        ]);

        assert_eq!(result_type.len(), 2);
        assert!(!result_type.is_empty());
        assert_eq!(
            result_type.kinds(),
            &[
                ValueType::Number(NumberType::I64),
                ValueType::Number(NumberType::F64),
            ]
        );
    }

    #[test]
    fn new_limit() {
        let max = Some(2);
        let min = 0;
        let limit = Limit::new(min, max);

        assert_eq!(limit.min, min);
        assert_eq!(limit.max, max);
    }

    #[test]
    fn new_memory_type() {
        let limit = Limit::new(0, None);
        let memory_type = MemoryType::new(limit.clone());

        assert_eq!(memory_type.limits(), &limit);
    }

    #[test]
    fn new_table_type() {
        let limit = Limit::new(0, None);
        let reference_type = ReferenceType::External;
        let table_type = TableType::new(limit.clone(), reference_type);

        assert_eq!(table_type.limits(), &limit);
        assert_eq!(table_type.kind(), &reference_type);
    }

    #[test]
    fn new_global_type() {
        let is_mutable = true;
        let kind = ValueType::Number(NumberType::I64);
        let global_type = GlobalType::new(is_mutable, kind);

        assert_eq!(global_type.is_mutable(), is_mutable);
        assert_eq!(global_type.kind(), &kind);
    }
}
