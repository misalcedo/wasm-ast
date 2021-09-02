//! Definitions are referenced with zero-based indices.
//! Each class of definition has its own index space, as distinguished by the following classes.
//!
//! The index space for functions, tables,
//! memories and globals includes respective imports declared in the same module.
//! The indices of these imports precede the indices of other definitions in the same index space.
//!
//! Element indices reference element segments and data indices reference data segments.
//!
//! The index space for locals is only accessible inside a function and includes the parameters of that function,
//! which precede the local variables.
//!
//! Label indices reference structured control instructions inside an instruction sequence.
//!
//! See <https://webassembly.github.io/spec/core/syntax/modules.html#indices>

pub type TypeIndex = usize;
pub type FunctionIndex = usize;
pub type TableIndex = usize;
pub type MemoryIndex = usize;
pub type GlobalIndex = usize;
pub type ElementIndex = usize;
pub type DataIndex = usize;
pub type LocalIndex = usize;
pub type LabelIndex = usize;
