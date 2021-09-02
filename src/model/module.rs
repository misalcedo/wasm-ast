//! WebAssembly model of modules and their segments.

use crate::model::indices::*;
use crate::model::types::*;
use crate::model::{Expression, Name};
use std::collections::HashMap;

/// A builder pattern for `Module`s.
pub struct ModuleBuilder {
    module: Module,
}

impl ModuleBuilder {
    /// Creates a new empty builder of WebAssembly modules.
    pub fn new() -> Self {
        ModuleBuilder {
            module: Module::empty(),
        }
    }

    /// Sets the function types segment for the WebAssembly module to be built.
    pub fn set_function_types(&mut self, function_types: Vec<FunctionType>) {
        self.module.function_types = Some(function_types);
    }

    /// Sets the functions segment for the WebAssembly module to be built.
    pub fn set_functions(&mut self, functions: Vec<Function>) {
        self.module.functions = Some(functions);
    }

    /// Sets the table segment for the WebAssembly module to be built.
    pub fn set_tables(&mut self, tables: Vec<Table>) {
        self.module.tables = Some(tables);
    }

    /// Sets the tables segment for the WebAssembly module to be built.
    pub fn set_memories(&mut self, memories: Vec<Memory>) {
        self.module.memories = Some(memories);
    }

    /// Sets the globals segment for the WebAssembly module to be built.
    pub fn set_globals(&mut self, globals: Vec<Global>) {
        self.module.globals = Some(globals);
    }

    /// Sets the elements segment for the WebAssembly module to be built.
    pub fn set_elements(&mut self, elements: Vec<Element>) {
        self.module.elements = Some(elements);
    }

    /// Sets the data segment for the WebAssembly module to be built.
    pub fn set_data(&mut self, data: Vec<Data>) {
        self.module.data = Some(data);
    }

    /// Sets the start segment for the WebAssembly module to be built.
    pub fn set_start(&mut self, start: Start) {
        self.module.start = Some(start);
    }

    /// Sets the imports segment for the WebAssembly module to be built.
    pub fn set_imports(&mut self, imports: Vec<Import>) {
        self.module.imports = Some(imports);
    }

    /// Sets the exports segment for the WebAssembly module to be built.
    pub fn set_exports(&mut self, exports: Vec<Export>) {
        self.module.exports = Some(exports);
    }

    /// Sets the custom section at the given insertion point for the WebAssembly module to be built.
    /// WebAssembly binary format allows custom sections to be at the start of a module, or after any other section.
    pub fn set_custom_sections(
        &mut self,
        insertion_point: ModuleSection,
        custom_sections: Vec<Custom>,
    ) {
        self.module.customs.insert(insertion_point, custom_sections);
    }

    /// Determines whether the WebAssembly module to be built will include a data count section or not.  
    pub fn include_data_count(&mut self, include: bool) {
        self.module.data_count = include;
    }

    /// Builds the current segments into a module.
    pub fn build(self) -> Module {
        self.into()
    }
}

impl From<ModuleBuilder> for Module {
    fn from(builder: ModuleBuilder) -> Self {
        builder.module
    }
}

impl Default for ModuleBuilder {
    fn default() -> Self {
        ModuleBuilder {
            module: Module::empty(),
        }
    }
}

/// WebAssembly programs are organized into modules, which are the unit of deployment, loading, and compilation.
/// A module collects definitions for types, functions, tables, memories, and globals.
/// In addition,
/// it can declare imports and exports and provide initialization in the form of data and element segments,
/// or a start function.
/// Each of the vectors – and thus the entire module – may be empty.
///
/// See <https://webassembly.github.io/spec/core/syntax/modules.html#modules>
///
/// # Examples
/// ## Empty
/// ```rust
/// use wasm_ast::{Module, ModuleSection};
/// use std::collections::HashMap;
///
/// let module = Module::empty();
///
/// assert_eq!(module.functions(), None);
/// assert_eq!(module.functions(), None);
/// assert_eq!(module.tables(), None);
/// assert_eq!(module.memories(), None);
/// assert_eq!(module.globals(), None);
/// assert_eq!(module.elements(), None);
/// assert_eq!(module.data(), None);
/// assert_eq!(module.start(), None);
/// assert_eq!(module.imports(), None);
/// assert_eq!(module.exports(), None);
/// assert_eq!(module.include_data_count(), false);
/// ```
///
/// ## Builder
/// ```rust
/// use wasm_ast::{Module, Import, FunctionType, ValueType, Start, Function, ResultType, ControlInstruction, Memory, Limit, Export, Data, Expression, ModuleSection, Custom};
/// use std::collections::HashMap;
///
/// let mut module = Module::builder();
/// let module = module.build();
///
/// assert_eq!(module.functions(), None);
/// assert_eq!(module.functions(), None);
/// assert_eq!(module.tables(), None);
/// assert_eq!(module.memories(), None);
/// assert_eq!(module.globals(), None);
/// assert_eq!(module.elements(), None);
/// assert_eq!(module.data(), None);
/// assert_eq!(module.start(), None);
/// assert_eq!(module.imports(), None);
/// assert_eq!(module.exports(), None);
/// assert_eq!(module.include_data_count(), false);
/// ```
#[derive(Clone, Debug)]
pub struct Module {
    function_types: Option<Vec<FunctionType>>,
    functions: Option<Vec<Function>>,
    tables: Option<Vec<Table>>,
    memories: Option<Vec<Memory>>,
    globals: Option<Vec<Global>>,
    elements: Option<Vec<Element>>,
    data: Option<Vec<Data>>,
    start: Option<Start>,
    imports: Option<Vec<Import>>,
    exports: Option<Vec<Export>>,
    customs: HashMap<ModuleSection, Vec<Custom>>,
    data_count: bool,
}

impl Module {
    /// Creates a builder for WebAssembly modules.
    pub fn builder() -> ModuleBuilder {
        ModuleBuilder::new()
    }

    /// Creates a new empty `Module`.
    pub fn empty() -> Self {
        Module {
            function_types: None,
            functions: None,
            tables: None,
            memories: None,
            globals: None,
            elements: None,
            data: None,
            start: None,
            imports: None,
            exports: None,
            customs: HashMap::new(),
            data_count: false,
        }
    }

    /// The 𝗍𝗒𝗉𝖾𝗌 component of a module defines a vector of function types.
    pub fn function_types(&self) -> Option<&[FunctionType]> {
        self.function_types.as_deref()
    }

    /// The 𝖿𝗎𝗇𝖼𝗌 component of a module defines a vector of functions.
    pub fn functions(&self) -> Option<&[Function]> {
        self.functions.as_deref()
    }

    /// The 𝗍𝖺𝖻𝗅𝖾𝗌 component of a module defines a vector of tables described by their table type.
    pub fn tables(&self) -> Option<&[Table]> {
        self.tables.as_deref()
    }

    /// The 𝗆𝖾𝗆𝗌 component of a module defines a vector of linear memories (or memories for short)
    /// as described by their memory type.
    pub fn memories(&self) -> Option<&[Memory]> {
        self.memories.as_deref()
    }

    /// The 𝗀𝗅𝗈𝖻𝖺𝗅𝗌 component of a module defines a vector of global variables (or globals for short).
    pub fn globals(&self) -> Option<&[Global]> {
        self.globals.as_deref()
    }

    /// The 𝖾𝗅𝖾𝗆𝗌 component of a module defines a vector of element segments.
    pub fn elements(&self) -> Option<&[Element]> {
        self.elements.as_deref()
    }

    /// The 𝖽𝖺𝗍𝖺𝗌 component of a module defines a vector of data segments.
    pub fn data(&self) -> Option<&[Data]> {
        self.data.as_deref()
    }

    /// The 𝗌𝗍𝖺𝗋𝗍 component of a module declares the function index of a start function that is
    /// automatically invoked when the module is instantiated, after tables and memories have been initialized.
    pub fn start(&self) -> Option<&Start> {
        self.start.as_ref()
    }

    /// The 𝗂𝗆𝗉𝗈𝗋𝗍𝗌 component of a module defines a set of imports that are required for instantiation.
    pub fn imports(&self) -> Option<&[Import]> {
        self.imports.as_deref()
    }

    /// The 𝖾𝗑𝗉𝗈𝗋𝗍𝗌 component of a module defines a set of exports that become accessible to the
    /// host environment once the module has been instantiated.
    pub fn exports(&self) -> Option<&[Export]> {
        self.exports.as_deref()
    }

    /// The custom sections of a module for a given insertion point.
    /// Custom sections are allowed at the beginning of a module and after every other section.
    pub fn custom_sections_at(&self, insertion_point: ModuleSection) -> Option<&[Custom]> {
        self.customs.get(&insertion_point).map(Vec::as_slice)
    }

    /// Whether the module includes the data count section or not.
    pub fn include_data_count(&self) -> bool {
        self.data_count
    }
}

/// Custom sections have the id 0.
/// They are intended to be used for debugging information or third-party extensions,
/// and are ignored by the WebAssembly semantics. Their contents consist of a name further
/// identifying the custom section, followed by an uninterpreted sequence of bytes for custom use.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html#binary-customsec>
///
/// # Examples
/// ```rust
/// use wasm_ast::{Custom, Name};
///
/// let name = "version";
/// let version = b"1.0.0";
/// let custom = Custom::new(name.into(), version.to_vec());
///
/// assert_eq!(custom.name(), &Name::new(name.to_string()));
/// assert_eq!(custom.bytes(), &version[..]);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Custom {
    name: Name,
    bytes: Vec<u8>,
}

impl Custom {
    /// Creates a new instance of a custom section.
    pub fn new(name: Name, bytes: Vec<u8>) -> Self {
        Custom { name, bytes }
    }

    /// The name of the custom section.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// The contents of the custom section.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// The 𝗍𝗒𝗉𝖾 of a function declares its signature by reference to a type defined in the module.
/// The parameters of the function are referenced through 0-based local indices in the function’s body; they are mutable.
/// The 𝗅𝗈𝖼𝖺𝗅𝗌 declare a vector of mutable local variables and their types.
/// These variables are referenced through local indices in the function’s body.
/// The index of the first local is the smallest index not referencing a parameter.
/// The 𝖻𝗈𝖽𝗒 is an instruction sequence that upon termination must produce a stack matching the function type’s result type.
///
/// See <https://webassembly.github.io/spec/core/syntax/modules.html#functions>
///
/// # Examples
/// ```rust
/// use wasm_ast::{Function, TypeIndex, Expression, ResultType, ValueType, NumericInstruction, NumberType};
///
/// let locals: ResultType = vec![ValueType::I32, ValueType::F32].into();
/// let body: Expression = vec![
///     32u32.into(),
///     2u32.into(),
///     NumericInstruction::Multiply(NumberType::I32).into()
/// ].into();
/// let function = Function::new(0, locals.clone(), body.clone());
///
/// assert_eq!(function.kind(), 0);
/// assert_eq!(function.locals(), &locals);
/// assert_eq!(function.body(), &body);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    kind: TypeIndex,
    locals: ResultType,
    body: Expression,
}

impl Function {
    pub fn new(kind: TypeIndex, locals: ResultType, body: Expression) -> Self {
        Function { kind, locals, body }
    }

    /// The index of the type definition for this `Function`.
    pub fn kind(&self) -> TypeIndex {
        self.kind
    }

    /// The types of the locals of this `Function`.
    pub fn locals(&self) -> &ResultType {
        &self.locals
    }

    /// The code for this `Function`.
    pub fn body(&self) -> &Expression {
        &self.body
    }
}

/// A table is a vector of opaque values of a particular reference type.
/// The 𝗆𝗂𝗇 size in the limits of the table type specifies the initial size of that table, while its 𝗆𝖺𝗑, if present, restricts the size to which it can grow later.
/// Tables can be initialized through element segments.
/// Tables are referenced through table indices,
/// starting with the smallest index not referencing a table import.
/// Most constructs implicitly reference table index 0.
///
/// See <https://webassembly.github.io/spec/core/syntax/modules.html#tables>
///
/// # Examples
/// ```rust
/// use wasm_ast::{Table, TableType, Limit, ReferenceType};
///
/// let limit = Limit::bounded(1, 2);
/// let kind = TableType::new(limit, ReferenceType::Function);
/// let table = Table::new(kind);
///
/// assert_eq!(table, kind.into());
/// assert_eq!(table.kind(), &kind);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Table {
    kind: TableType,
}

impl Table {
    /// Creates a new instance of a `Table`.
    pub fn new(kind: TableType) -> Self {
        Table { kind }
    }

    /// The type descriptor of this `Table`.
    pub fn kind(&self) -> &TableType {
        &self.kind
    }
}

impl From<TableType> for Table {
    fn from(kind: TableType) -> Self {
        Table { kind }
    }
}

/// A memory is a vector of raw uninterpreted bytes.
/// The 𝗆𝗂𝗇 size in the limits of the memory type specifies the initial size of that memory,
/// while its 𝗆𝖺𝗑, if present, restricts the size to which it can grow later. Both are in units of page size.
/// Memories can be initialized through data segments.
/// Memories are referenced through memory indices
/// starting with the smallest index not referencing a memory import.
/// Most constructs implicitly reference memory index 0.
///
/// See <https://webassembly.github.io/spec/core/syntax/modules.html#memories>
///
/// # Examples
/// ```rust
/// use wasm_ast::{Memory, MemoryType, Limit};
///
/// let limit = Limit::bounded(1, 2);
/// let kind = MemoryType::new(limit);
/// let memory = Memory::new(kind);
///
/// assert_eq!(memory, kind.into());
/// assert_eq!(memory, limit.into());
/// assert_eq!(memory.kind(), &kind);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Memory {
    kind: MemoryType,
}

impl Memory {
    /// Creates a new `Memory` of the given type.
    pub fn new(kind: MemoryType) -> Self {
        Memory { kind }
    }

    /// The type definition for this memory.
    pub fn kind(&self) -> &MemoryType {
        &self.kind
    }
}

impl<T> From<T> for Memory
where
    T: Into<MemoryType>,
{
    fn from(kind: T) -> Self {
        Memory { kind: kind.into() }
    }
}

/// Each global stores a single value of the given global type.
/// Its 𝗍𝗒𝗉𝖾 also specifies whether a global is immutable or mutable.
/// Moreover, each global is initialized with an 𝗂𝗇𝗂𝗍 value given by a constant initializer expression.
/// Globals are referenced through global indices,
/// starting with the smallest index not referencing a global import.
///
/// See <https://webassembly.github.io/spec/core/syntax/modules.html#globals>
///
/// # Examples
/// ## Immutable
/// ```rust
/// use wasm_ast::{Global, GlobalType, ValueType, Expression};
///
/// let kind = GlobalType::immutable(ValueType::I64);
/// let initializer: Expression = vec![0i64.into()].into();
/// let global = Global::immutable(ValueType::I64, initializer.clone());
///
/// assert_eq!(global, Global::new(kind.clone(), initializer.clone()));
/// assert_eq!(global.kind(), &kind);
/// assert_eq!(global.initializer(), &initializer);
/// ```
///
/// ## Mutable
/// ```rust
/// use wasm_ast::{Global, GlobalType, ValueType, Expression};
///
/// let kind = GlobalType::mutable(ValueType::I64);
/// let initializer: Expression = vec![0i64.into()].into();
/// let global = Global::mutable(ValueType::I64, initializer.clone());
///
/// assert_eq!(global, Global::new(kind.clone(), initializer.clone()));
/// assert_eq!(global.kind(), &kind);
/// assert_eq!(global.initializer(), &initializer);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Global {
    kind: GlobalType,
    initializer: Expression,
}

impl Global {
    /// Creates a new `Global` with the given type and initializer.
    pub fn new(kind: GlobalType, initializer: Expression) -> Self {
        Global { kind, initializer }
    }

    /// Creates a new `Global` for a mutable global variable.
    pub fn mutable(kind: ValueType, initializer: Expression) -> Self {
        Global {
            kind: GlobalType::mutable(kind),
            initializer,
        }
    }

    /// Creates a new `Global` for an immutable global variable.
    pub fn immutable(kind: ValueType, initializer: Expression) -> Self {
        Global {
            kind: GlobalType::immutable(kind),
            initializer,
        }
    }

    /// The type of this `Global`.
    pub fn kind(&self) -> &GlobalType {
        &self.kind
    }

    /// The expression to initialize this `Global` with.
    pub fn initializer(&self) -> &Expression {
        &self.initializer
    }
}

/// The initial contents of a table is uninitialized.
/// Element segments can be used to initialize a subrange of a table from a static vector of elements.
/// Each element segment defines a reference type and a corresponding list of constant element expressions.
/// Element segments have a mode that identifies them as either passive, active, or declarative.
/// A passive element segment’s elements can be copied to a table using the 𝗍𝖺𝖻𝗅𝖾.𝗂𝗇𝗂𝗍 instruction.
/// An active element segment copies its elements into a table during instantiation,
/// as specified by a table index and a constant expression defining an offset into that table.
/// A declarative element segment is not available at runtime but merely serves to forward-declare
/// references that are formed in code with instructions like 𝗋𝖾𝖿.𝖿𝗎𝗇𝖼.
/// The 𝗈𝖿𝖿𝗌𝖾𝗍 is given by a constant expression.
/// Element segments are referenced through element indices.
///
/// See <https://webassembly.github.io/spec/core/syntax/modules.html#element-segments>
///
/// # Examples
/// ## Active
/// ```rust
/// use wasm_ast::{Element, ElementInitializer, ElementMode, TableIndex, FunctionIndex, Expression, ReferenceType};
///
/// let offset: Expression = vec![0i32.into()].into();
/// let initializer: ElementInitializer = vec![0].into();
/// let element = Element::active(0, offset.clone(), ReferenceType::Function, initializer.clone());
///
/// assert_eq!(element, Element::new(
///     ReferenceType::Function,
///     ElementMode::Active(0, offset.clone()),
///     initializer.clone()
/// ));
/// assert_eq!(element.kind(), ReferenceType::Function);
/// assert_eq!(element.mode(), &ElementMode::Active(0, offset.clone()));
/// assert_eq!(element.initializers(), &initializer);
/// ```
///
/// ## Passive
/// ```rust
/// use wasm_ast::{Element, ElementInitializer, ElementMode, TableIndex, Expression, ReferenceType, NumericInstruction};
///
/// let initializer: ElementInitializer = vec![Expression::from(vec![2i32.into()])].into();
/// let element = Element::passive(ReferenceType::External, initializer.clone());
///
/// assert_eq!(element, Element::new(
///     ReferenceType::External,
///     ElementMode::Passive,
///     initializer.clone()
/// ));
/// assert_eq!(element.kind(), ReferenceType::External);
/// assert_eq!(element.mode(), &ElementMode::Passive);
/// assert_eq!(element.initializers(), &initializer);
/// ```
///
/// ## Declarative
/// ```rust
/// use wasm_ast::{Element, ElementInitializer, ElementMode, TableIndex, Expression, ReferenceType, NumericInstruction};
///
/// let initializer: ElementInitializer = vec![Expression::from(vec![2i32.into()])].into();
/// let element = Element::declarative(ReferenceType::External, initializer.clone());
///
/// assert_eq!(element, Element::new(
///     ReferenceType::External,
///     ElementMode::Declarative,
///     initializer.clone()
/// ));
/// assert_eq!(element.kind(), ReferenceType::External);
/// assert_eq!(element.mode(), &ElementMode::Declarative);
/// assert_eq!(element.initializers(), &initializer);
/// ````
#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    kind: ReferenceType,
    mode: ElementMode,
    initializers: ElementInitializer,
}

impl Element {
    /// Creates a new instance of an element segment.
    pub fn new(kind: ReferenceType, mode: ElementMode, initializers: ElementInitializer) -> Self {
        Element {
            kind,
            mode,
            initializers,
        }
    }

    /// Creates a passive element segment.
    pub fn passive(kind: ReferenceType, initializers: ElementInitializer) -> Self {
        Element {
            kind,
            mode: ElementMode::Passive,
            initializers,
        }
    }

    /// Creates an active element segment.
    pub fn active(
        table: TableIndex,
        offset: Expression,
        kind: ReferenceType,
        initializers: ElementInitializer,
    ) -> Self {
        Element {
            kind,
            mode: ElementMode::Active(table, offset),
            initializers,
        }
    }

    /// Creates a declarative element segment.
    pub fn declarative(kind: ReferenceType, initializers: ElementInitializer) -> Self {
        Element {
            kind,
            mode: ElementMode::Declarative,
            initializers,
        }
    }

    /// The reference type of the element segment.
    pub fn kind(&self) -> ReferenceType {
        self.kind
    }

    /// The initializer for the element segment.
    pub fn initializers(&self) -> &ElementInitializer {
        &self.initializers
    }

    /// The mode of the element segment.
    pub fn mode(&self) -> &ElementMode {
        &self.mode
    }
}

/// The specification only describes elements as allowing expressions for the initializer.
/// However, the binary specification allows a vector of function indices.
/// We need to deviate from the specification here in order to support the full binary format.
#[derive(Clone, Debug, PartialEq)]
pub enum ElementInitializer {
    Expression(Vec<Expression>),
    FunctionIndex(Vec<FunctionIndex>),
}

impl From<Vec<Expression>> for ElementInitializer {
    fn from(expressions: Vec<Expression>) -> Self {
        ElementInitializer::Expression(expressions)
    }
}

impl From<Vec<FunctionIndex>> for ElementInitializer {
    fn from(indices: Vec<FunctionIndex>) -> Self {
        ElementInitializer::FunctionIndex(indices)
    }
}

/// Element segments have a mode that identifies them as either passive, active, or declarative.
#[derive(Clone, Debug, PartialEq)]
pub enum ElementMode {
    /// A passive element segment’s elements can be copied to a table using the 𝗍𝖺𝖻𝗅𝖾.𝗂𝗇𝗂𝗍 instruction.
    Passive,
    /// An active element segment copies its elements into a table during instantiation,
    /// as specified by a table index and a constant expression defining an offset into that table.
    /// The 𝗈𝖿𝖿𝗌𝖾𝗍 is given by a constant expression.
    Active(TableIndex, Expression),
    /// A declarative element segment is not available at runtime but merely serves to forward-declare
    /// references that are formed in code with instructions like 𝗋𝖾𝖿.𝖿𝗎𝗇𝖼.
    Declarative,
}

/// The initial contents of a memory are zero bytes.
/// Data segments can be used to initialize a range of memory from a static vector of bytes.
/// Like element segments, data segments have a mode that identifies them as either passive or active.
/// A passive data segment’s contents can be copied into a memory using the 𝗆𝖾𝗆𝗈𝗋𝗒.𝗂𝗇𝗂𝗍 instruction.
/// An active data segment copies its contents into a memory during instantiation,
/// as specified by a memory index and a constant expression defining an offset into that memory.
/// Data segments are referenced through data indices.
///
/// See <https://webassembly.github.io/spec/core/syntax/modules.html#data-segments>
///
/// # Examples
/// ## Passive
/// ```rust
/// use wasm_ast::{Data, DataMode};
///
/// let initializer = vec![42];
/// let data = Data::passive(initializer.clone());
///
/// assert_eq!(data, Data::new(DataMode::Passive, initializer.clone()));
/// assert_eq!(data, initializer.into());
/// assert_eq!(data.mode(), &DataMode::Passive);
/// assert_eq!(data.len(), 1);
/// assert_eq!(data.is_empty(), false);
/// ```
///
/// ## Active
/// ```rust
/// use wasm_ast::{Data, DataMode, MemoryIndex, Expression};
///
/// let initializer = vec![42];
/// let offset: Expression = vec![1u32.into()].into();
/// let data = Data::active(0, offset.clone(), initializer.clone());
///
/// assert_eq!(data, Data::new(DataMode::Active(0, offset.clone()), initializer.clone()));
/// assert_eq!(data.mode(), &DataMode::Active(0, offset));
/// assert_eq!(data.len(), 1);
/// assert_eq!(data.is_empty(), false);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    mode: DataMode,
    initializer: Vec<u8>,
}

impl Data {
    /// Creates an instance of a data segment.
    pub fn new(mode: DataMode, initializer: Vec<u8>) -> Self {
        Data { mode, initializer }
    }

    /// Creates an instance of a passive data segment.
    pub fn passive(initializer: Vec<u8>) -> Self {
        Data {
            mode: DataMode::Passive,
            initializer,
        }
    }

    /// Creates an instance of an active data segment.
    pub fn active(memory: MemoryIndex, offset: Expression, initializer: Vec<u8>) -> Self {
        Data {
            mode: DataMode::Active(memory, offset),
            initializer,
        }
    }

    /// The mode of the data segment.
    pub fn mode(&self) -> &DataMode {
        &self.mode
    }

    /// The data to initialize the segment with.
    pub fn initializer(&self) -> &[u8] {
        &self.initializer
    }

    /// The number of bytes in the data segment initializer.
    pub fn len(&self) -> usize {
        self.initializer.len()
    }

    /// True if the data segment's initializer's length is zero, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.initializer.is_empty()
    }
}

impl From<Vec<u8>> for Data {
    fn from(initializer: Vec<u8>) -> Self {
        Data {
            mode: DataMode::Passive,
            initializer,
        }
    }
}

/// Like element segments, data segments have a mode that identifies them as either passive or active.
#[derive(Clone, Debug, PartialEq)]
pub enum DataMode {
    /// A passive data segment’s contents can be copied into a memory using the 𝗆𝖾𝗆𝗈𝗋𝗒.𝗂𝗇𝗂𝗍 instruction.
    Passive,
    /// An active data segment copies its contents into a memory during instantiation,
    /// as specified by a memory index and a constant expression defining an offset into that memory.
    Active(MemoryIndex, Expression),
}

/// The 𝗌𝗍𝖺𝗋𝗍 component of a module declares the function index of a start function that
/// is automatically invoked when the module is instantiated,
/// after tables and memories have been initialized.
/// start::={𝖿𝗎𝗇𝖼 funcidx}
///
/// See <https://webassembly.github.io/spec/core/syntax/modules.html#start-function>
///
/// # Examples
/// ```rust
/// use wasm_ast::Start;
///
/// assert_eq!(Start::new(0).function(), 0);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Start {
    function: FunctionIndex,
}

impl Start {
    /// Creates a new instance of `Start` referencing the given function.
    pub fn new(function: FunctionIndex) -> Self {
        Start { function }
    }

    /// The index of the function to run at module instantiation.
    pub fn function(&self) -> FunctionIndex {
        self.function
    }
}

/// The 𝖾𝗑𝗉𝗈𝗋𝗍𝗌 component of a module defines a set of exports that become accessible to the
/// host environment once the module has been instantiated.
/// Each export is labeled by a unique name.
/// Exportable definitions are functions, tables, memories, and globals,
/// which are referenced through a respective descriptor.
///
/// See <https://webassembly.github.io/spec/core/syntax/modules.html#exports>
///
/// # Examples
/// ## Table
/// ```rust
/// use wasm_ast::{Export, ExportDescription, Name};
///
/// let name = "functions";
/// let description = ExportDescription::Table(0);
/// let export = Export::new(name.into(), description.clone());
///
/// assert_eq!(export, Export::table(name.into(), 0));
/// assert_eq!(export.name(), &Name::new(String::from(name)));
/// assert_eq!(export.description(), &description);
/// ```
///
/// ## Memory
/// ```rust
/// use wasm_ast::{Export, ExportDescription, Name};
///
/// let name = "io";
/// let description = ExportDescription::Memory(1);
/// let export = Export::new(name.into(), description.clone());
///
/// assert_eq!(export, Export::memory(name.into(), 1));
/// assert_eq!(export.name(), &Name::new(String::from(name)));
/// assert_eq!(export.description(), &description);
/// ```
///
/// ## Function
/// ```rust
/// use wasm_ast::{Export, ExportDescription, Name};
///
/// let name = "print";
/// let description = ExportDescription::Function(42);
/// let export = Export::new(name.into(), description.clone());
///
/// assert_eq!(export, Export::function(name.into(), 42));
/// assert_eq!(export.name(), &Name::new(String::from(name)));
/// assert_eq!(export.description(), &description);
/// ```
///
/// ## Global
/// ```rust
/// use wasm_ast::{Export, ExportDescription, Name};
///
/// let name = "functions";
/// let description = ExportDescription::Global(2);
/// let export = Export::new(name.into(), description.clone());
///
/// assert_eq!(export, Export::global(name.into(), 2));
/// assert_eq!(export.name(), &Name::new(String::from(name)));
/// assert_eq!(export.description(), &description);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Export {
    name: Name,
    description: ExportDescription,
}

impl Export {
    /// Create a new instance of an `Export` with the given name and description.
    pub fn new(name: Name, description: ExportDescription) -> Self {
        Export { name, description }
    }

    /// Create a new instance of an `Export` with the given name and description for a table.
    pub fn table(name: Name, table: TableIndex) -> Self {
        Export {
            name,
            description: ExportDescription::Table(table),
        }
    }

    /// Create a new instance of an `Export` with the given name and description for a memory.
    pub fn memory(name: Name, memory: MemoryIndex) -> Self {
        Export {
            name,
            description: ExportDescription::Memory(memory),
        }
    }

    /// Create a new instance of an `Export` with the given name and description for a function.
    pub fn function(name: Name, function: FunctionIndex) -> Self {
        Export {
            name,
            description: ExportDescription::Function(function),
        }
    }

    /// Create a new instance of an `Export` with the given name and description for a global.
    pub fn global(name: Name, global: GlobalIndex) -> Self {
        Export {
            name,
            description: ExportDescription::Global(global),
        }
    }

    /// The name of the export.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// The description of the table.
    pub fn description(&self) -> &ExportDescription {
        &self.description
    }
}

/// Exportable definitions are functions, tables, memories, and globals,
/// which are referenced through a respective descriptor.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ExportDescription {
    Function(FunctionIndex),
    Table(TableIndex),
    Memory(MemoryIndex),
    Global(GlobalIndex),
}

/// Each import is labeled by a two-level name space,
/// consisting of a 𝗆𝗈𝖽𝗎𝗅𝖾 name and a 𝗇𝖺𝗆𝖾 for an entity within that module.
/// Importable definitions are functions, tables, memories, and globals.
/// Each import is specified by a descriptor with a respective type that a definition provided
/// during instantiation is required to match.
/// Every import defines an index in the respective index space.
/// In each index space, the indices of imports go before the first index of any
/// definition contained in the module itself.
///
/// See <https://webassembly.github.io/spec/core/syntax/modules.html#imports>
///
/// # Examples
/// ## Table
/// ```rust
/// use wasm_ast::{Import, ImportDescription, Name, TableType, Limit, ReferenceType};
///
/// let module = "system";
/// let name = "functions";
/// let kind = TableType::new(Limit::unbounded(1), ReferenceType::Function);
/// let description = ImportDescription::Table(kind.clone());
/// let import = Import::new(module.into(), name.into(), description.clone());
///
/// assert_eq!(import, Import::table(module.into(), name.into(), kind));
/// assert_eq!(import.module(), &Name::new(String::from(module)));
/// assert_eq!(import.name(), &Name::new(String::from(name)));
/// assert_eq!(import.description(), &description);
/// ```
///
/// ## Memory
/// ```rust
/// use wasm_ast::{Import, ImportDescription, Name, MemoryType, Limit};
///
/// let module = "system";
/// let name = "io";
/// let kind = Limit::bounded(1, 2).into();
/// let description = ImportDescription::Memory(kind);
/// let import = Import::new(module.into(), name.into(), description.clone());
///
/// assert_eq!(import, Import::memory(module.into(), name.into(), kind));
/// assert_eq!(import.module(), &Name::new(String::from(module)));
/// assert_eq!(import.name(), &Name::new(String::from(name)));
/// assert_eq!(import.description(), &description);
/// ```
///
/// ## Function
/// ```rust
/// use wasm_ast::{Import, ImportDescription, Name};
///
/// let module = "system";
/// let name = "print";
/// let description = ImportDescription::Function(42);
/// let import = Import::new(module.into(), name.into(), description.clone());
///
/// assert_eq!(import, Import::function(module.into(), name.into(), 42));
/// assert_eq!(import.module(), &Name::new(String::from(module)));
/// assert_eq!(import.name(), &Name::new(String::from(name)));
/// assert_eq!(import.description(), &description);
/// ```
///
/// ## Global
/// ```rust
/// use wasm_ast::{Import, ImportDescription, Name, GlobalType, ValueType};
///
/// let module = "system";
/// let name = "counter";
/// let kind = GlobalType::mutable(ValueType::I64);
/// let description = ImportDescription::Global(kind.clone());
/// let import = Import::new(module.into(), name.into(), description.clone());
///
/// assert_eq!(import, Import::global(module.into(), name.into(), kind));
/// assert_eq!(import.module(), &Name::new(String::from(module)));
/// assert_eq!(import.name(), &Name::new(String::from(name)));
/// assert_eq!(import.description(), &description);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Import {
    module: Name,
    name: Name,
    description: ImportDescription,
}

impl Import {
    /// Creates a new import.
    pub fn new(module: Name, name: Name, description: ImportDescription) -> Self {
        Import {
            module,
            name,
            description,
        }
    }

    /// Create a new instance of an `Import` with the given name and description for a table.
    pub fn table(module: Name, name: Name, table_kind: TableType) -> Self {
        Import {
            module,
            name,
            description: ImportDescription::Table(table_kind),
        }
    }

    /// Create a new instance of an `Import` with the given name and description for a memory.
    pub fn memory(module: Name, name: Name, memory_kind: MemoryType) -> Self {
        Import {
            module,
            name,
            description: ImportDescription::Memory(memory_kind),
        }
    }

    /// Create a new instance of an `Import` with the given name and description for a function.
    pub fn function(module: Name, name: Name, function_kind: TypeIndex) -> Self {
        Import {
            module,
            name,
            description: ImportDescription::Function(function_kind),
        }
    }

    /// Create a new instance of an `Import` with the given name and description for a global.
    pub fn global(module: Name, name: Name, global_kind: GlobalType) -> Self {
        Import {
            module,
            name,
            description: ImportDescription::Global(global_kind),
        }
    }

    /// The name of the module (i.e.m namespace).
    pub fn module(&self) -> &Name {
        &self.module
    }

    /// The name of the import.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// The description of the import.
    pub fn description(&self) -> &ImportDescription {
        &self.description
    }
}

/// Each import is specified by a descriptor with a respective type that a definition provided
/// during instantiation is required to match.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ImportDescription {
    Function(TypeIndex),
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
}

/// The binary encoding of modules is organized into sections.
/// Most sections correspond to one component of a module record,
/// except that function definitions are split into two sections,
/// separating their type declarations in the function section from their bodies in the code section.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html>
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ModuleSection {
    /// Custom sections have the id 0.
    /// They are intended to be used for debugging information or third-party extensions,
    /// and are ignored by the WebAssembly semantics.
    /// Their contents consist of a name further identifying the custom section,
    /// followed by an uninterpreted sequence of bytes for custom use.
    Custom = 0,
    /// The type section has the id 1.
    /// It decodes into a vector of function types that represent the 𝗍𝗒𝗉𝖾𝗌 component of a module.
    Type,
    /// The import section has the id 2.
    /// It decodes into a vector of imports that represent the 𝗂𝗆𝗉𝗈𝗋𝗍𝗌 component of a module.
    Import,
    /// The function section has the id 3.
    /// It decodes into a vector of type indices that represent the 𝗍𝗒𝗉𝖾 fields of the functions
    /// in the 𝖿𝗎𝗇𝖼𝗌 component of a module. The 𝗅𝗈𝖼𝖺𝗅𝗌 and 𝖻𝗈𝖽𝗒 fields of the respective functions
    /// are encoded separately in the code section.
    Function,
    /// The table section has the id 4.
    /// It decodes into a vector of tables that represent the 𝗍𝖺𝖻𝗅𝖾𝗌 component of a module.
    Table,
    /// The memory section has the id 5.
    /// It decodes into a vector of memories that represent the 𝗆𝖾𝗆𝗌 component of a module.
    Memory,
    /// The global section has the id 6.
    /// It decodes into a vector of globals that represent the 𝗀𝗅𝗈𝖻𝖺𝗅𝗌 component of a module.
    Global,
    /// The export section has the id 7.
    /// It decodes into a vector of exports that represent the 𝖾𝗑𝗉𝗈𝗋𝗍𝗌 component of a module.
    Export,
    /// The start section has the id 8.
    /// It decodes into an optional start function that represents the 𝗌𝗍𝖺𝗋𝗍 component of a module.
    Start,
    /// The element section has the id 9.
    /// It decodes into a vector of element segments that represent the 𝖾𝗅𝖾𝗆𝗌 component of a module.
    Element,
    /// The code section has the id 10.
    /// It decodes into a vector of code entries that are pairs of value type vectors and expressions.
    /// They represent the 𝗅𝗈𝖼𝖺𝗅𝗌 and 𝖻𝗈𝖽𝗒 field of the functions in the 𝖿𝗎𝗇𝖼𝗌 component of a module.
    /// The 𝗍𝗒𝗉𝖾 fields of the respective functions are encoded separately in the function section.
    Code,
    /// The data section has the id 11.
    /// It decodes into a vector of data segments that represent the 𝖽𝖺𝗍𝖺𝗌 component of a module.
    Data,
    /// The data count section has the id 12.
    /// It decodes into an optional u32 that represents the number of data segments in the data section.
    /// If this count does not match the length of the data segment vector, the module is malformed.
    DataCount,
}
