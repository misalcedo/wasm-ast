use crate::model::types::*;
use crate::model::{Expression, Name};
use std::mem::discriminant;

/// WebAssembly programs are organized into modules, which are the unit of deployment, loading, and compilation.
/// A module collects definitions for types, functions, tables, memories, and globals.
/// In addition,
/// it can declare imports and exports and provide initialization in the form of data and element segments,
/// or a start function.
/// Each of the vectors – and thus the entire module – may be empty.
///
/// See https://webassembly.github.io/spec/core/syntax/modules.html#modules
#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    function_types: Vec<FunctionType>,
    functions: Vec<Function>,
    tables: Vec<Table>,
    memories: Vec<Memory>,
    globals: Vec<Global>,
    elements: Vec<Element>,
    data: Vec<Data>,
    start: Option<Start>,
    imports: Vec<Import>,
    exports: Vec<Export>,
}

impl Module {
    /// Creates a new empty `Module`.
    pub fn new() -> Self {
        Module {
            function_types: vec![],
            functions: vec![],
            tables: vec![],
            memories: vec![],
            globals: vec![],
            elements: vec![],
            data: vec![],
            start: None,
            imports: vec![],
            exports: vec![],
        }
    }

    /// The 𝗍𝗒𝗉𝖾𝗌 component of a module defines a vector of function types.
    pub fn types(&self) -> &[FunctionType] {
        &self.function_types
    }

    pub fn add_type(&mut self, function_type: FunctionType) -> TypeIndex {
        self.function_types.push(function_type);
        self.function_types.len() - 1
    }

    /// The 𝖿𝗎𝗇𝖼𝗌 component of a module defines a vector of functions.
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    /// The 𝗍𝖺𝖻𝗅𝖾𝗌 component of a module defines a vector of tables described by their table type.
    pub fn tables(&self) -> &[Table] {
        &self.tables
    }

    pub fn add_table(&mut self, table: Table) -> TableIndex {
        self.tables.push(table);

        let imported_tables = self
            .imports()
            .iter()
            .filter(|import| matches!(import.description(), ImportDescription::Table(_)))
            .count();

        imported_tables + self.tables.len() - 1
    }

    /// The 𝗆𝖾𝗆𝗌 component of a module defines a vector of linear memories (or memories for short)
    /// as described by their memory type.
    pub fn memories(&self) -> &[Memory] {
        &self.memories
    }

    pub fn add_memory(&mut self, memory: Memory) -> MemoryIndex {
        self.memories.push(memory);

        let imported_memories = self
            .imports()
            .iter()
            .filter(|import| matches!(import.description(), ImportDescription::Memory(_)))
            .count();

        imported_memories + self.memories.len() - 1
    }

    /// The 𝗀𝗅𝗈𝖻𝖺𝗅𝗌 component of a module defines a vector of global variables (or globals for short).
    pub fn globals(&self) -> &[Global] {
        &self.globals
    }

    pub fn add_global(&mut self, global: Global) {
        self.globals.push(global);
    }

    /// The 𝖾𝗅𝖾𝗆𝗌 component of a module defines a vector of element segments.
    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    pub fn add_element(&mut self, element: Element) -> ElementIndex {
        self.elements.push(element);
        self.elements().len() - 1
    }

    /// The 𝖽𝖺𝗍𝖺𝗌 component of a module defines a vector of data segments.
    pub fn data(&self) -> &[Data] {
        &self.data
    }

    pub fn add_data(&mut self, data: Data) {
        self.data.push(data);
    }

    /// The 𝗌𝗍𝖺𝗋𝗍 component of a module declares the function index of a start function that is
    /// automatically invoked when the module is instantiated, after tables and memories have been initialized.
    pub fn start(&self) -> Option<&Start> {
        self.start.as_ref()
    }

    pub fn set_start(&mut self, start: Option<Start>) {
        self.start = start;
    }

    /// The 𝗂𝗆𝗉𝗈𝗋𝗍𝗌 component of a module defines a set of imports that are required for instantiation.
    pub fn imports(&self) -> &[Import] {
        &self.imports
    }

    pub fn add_import(&mut self, import: Import) -> usize {
        let import_discriminant = discriminant(import.description());

        self.imports.push(import);

        self.imports()
            .iter()
            .filter(|i| discriminant(i.description()) == import_discriminant)
            .count()
            - 1
    }

    /// The 𝖾𝗑𝗉𝗈𝗋𝗍𝗌 component of a module defines a set of exports that become accessible to the
    /// host environment once the module has been instantiated.
    pub fn exports(&self) -> &[Export] {
        &self.exports
    }

    pub fn add_export(&mut self, export: Export) {
        self.exports.push(export);
    }
}

impl Default for Module {
    fn default() -> Self {
        Self::new()
    }
}

/// Definitions are referenced with zero-based indices.
/// Each class of definition has its own index space, as distinguished by the following classes.
///
/// The index space for functions, tables,
/// memories and globals includes respective imports declared in the same module.
/// The indices of these imports precede the indices of other definitions in the same index space.
///
/// Element indices reference element segments and data indices reference data segments.
///
/// The index space for locals is only accessible inside a function and includes the parameters of that function,
/// which precede the local variables.
///
/// Label indices reference structured control instructions inside an instruction sequence.
/// See https://webassembly.github.io/spec/core/syntax/modules.html#indices

pub type TypeIndex = usize;
pub type FunctionIndex = usize;
pub type TableIndex = usize;
pub type MemoryIndex = usize;
pub type GlobalIndex = usize;
pub type ElementIndex = usize;
pub type DataIndex = usize;
pub type LocalIndex = usize;
pub type LabelIndex = usize;

/// The 𝗍𝗒𝗉𝖾 of a function declares its signature by reference to a type defined in the module.
/// The parameters of the function are referenced through 0-based local indices in the function’s body; they are mutable.
/// The 𝗅𝗈𝖼𝖺𝗅𝗌 declare a vector of mutable local variables and their types.
/// These variables are referenced through local indices in the function’s body.
/// The index of the first local is the smallest index not referencing a parameter.
/// The 𝖻𝗈𝖽𝗒 is an instruction sequence that upon termination must produce a stack matching the function type’s result type.
///
/// See https://webassembly.github.io/spec/core/syntax/modules.html#functions
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

    pub fn kind(&self) -> TypeIndex {
        self.kind
    }

    pub fn locals(&self) -> &ResultType {
        &self.locals
    }

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
/// See https://webassembly.github.io/spec/core/syntax/modules.html#tables
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Table {
    kind: TableType,
}

impl Table {
    pub fn new(kind: TableType) -> Self {
        Table { kind }
    }

    pub fn kind(&self) -> &TableType {
        &self.kind
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
/// See https://webassembly.github.io/spec/core/syntax/modules.html#memories
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Memory {
    kind: MemoryType,
}

impl Memory {
    pub fn new(kind: MemoryType) -> Self {
        Memory { kind }
    }

    pub fn kind(&self) -> &MemoryType {
        &self.kind
    }
}

/// Each global stores a single value of the given global type.
/// Its 𝗍𝗒𝗉𝖾 also specifies whether a global is immutable or mutable.
/// Moreover, each global is initialized with an 𝗂𝗇𝗂𝗍 value given by a constant initializer expression.
/// Globals are referenced through global indices,
/// starting with the smallest index not referencing a global import.
///
/// See https://webassembly.github.io/spec/core/syntax/modules.html#globals
#[derive(Clone, Debug, PartialEq)]
pub struct Global {
    kind: GlobalType,
    initializer: Expression,
}

impl Global {
    pub fn new(kind: GlobalType, initializer: Expression) -> Self {
        Global { kind, initializer }
    }

    pub fn kind(&self) -> &GlobalType {
        &self.kind
    }

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
/// See https://webassembly.github.io/spec/core/syntax/modules.html#element-segments
#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    kind: ReferenceType,
    mode: ElementMode,
    initializers: ElementInitializer,
}

impl Element {
    pub fn new(kind: ReferenceType, mode: ElementMode, initializers: ElementInitializer) -> Self {
        Element {
            kind,
            mode,
            initializers,
        }
    }

    pub fn kind(&self) -> &ReferenceType {
        &self.kind
    }

    pub fn initializers(&self) -> &ElementInitializer {
        &self.initializers
    }

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
/// See https://webassembly.github.io/spec/core/syntax/modules.html#data-segments
#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    mode: DataMode,
    initializer: Vec<u8>,
}

impl Data {
    pub fn new(mode: DataMode, initializer: Vec<u8>) -> Self {
        Data { mode, initializer }
    }

    pub fn mode(&self) -> &DataMode {
        &self.mode
    }

    pub fn initializer(&self) -> &[u8] {
        &self.initializer
    }

    pub fn len(&self) -> usize {
        self.initializer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.initializer.is_empty()
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
/// See https://webassembly.github.io/spec/core/syntax/modules.html#start-function
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Start {
    function: FunctionIndex,
}

impl Start {
    pub fn new(function: FunctionIndex) -> Self {
        Start { function }
    }

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
/// See https://webassembly.github.io/spec/core/syntax/modules.html#exports
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Export {
    name: Name,
    description: ExportDescription,
}

impl Export {
    pub fn new(name: Name, description: ExportDescription) -> Self {
        Export { name, description }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

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
/// See https://webassembly.github.io/spec/core/syntax/modules.html#imports
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Import {
    module: Name,
    name: Name,
    description: ImportDescription,
}

impl Import {
    pub fn new(module: Name, name: Name, description: ImportDescription) -> Self {
        Import {
            module,
            name,
            description,
        }
    }

    pub fn module(&self) -> &Name {
        &self.module
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ControlInstruction, Instruction, NumericInstruction};

    #[test]
    fn empty_module() {
        let module = Module::new();

        assert!(module.types().is_empty());
        assert!(module.functions().is_empty());
        assert!(module.tables().is_empty());
        assert!(module.memories().is_empty());
        assert!(module.globals().is_empty());
        assert!(module.imports().is_empty());
        assert!(module.exports().is_empty());
        assert!(module.data().is_empty());
        assert!(module.elements().is_empty());
        assert!(module.start().is_none());
    }

    #[test]
    fn module() {
        let mut module = Module::new();
        let function_type = FunctionType::new(
            ResultType::from(vec![IntegerType::I64.into()]),
            ResultType::from(vec![FloatType::F64.into()]),
        );
        module.add_type(function_type.clone());

        let function = Function::new(
            0,
            ResultType::from(vec![IntegerType::I32.into()]),
            Expression::new(vec![Instruction::Control(ControlInstruction::Nop)]),
        );
        module.add_function(function.clone());

        let element = Element::new(
            ReferenceType::Function,
            ElementMode::Passive,
            ElementInitializer::FunctionIndex(vec![0]),
        );
        module.add_element(element.clone());

        let data = Data::new(DataMode::Passive, vec![42]);
        module.add_data(data.clone());

        let table = Table::new(TableType::new(Limit::new(0, None), ReferenceType::Function));
        module.add_table(table);

        let memory = Memory::new(MemoryType::from(Limit::new(0, None)));
        module.add_memory(memory);

        let import = Import::new(
            "test".into(),
            "foobar".into(),
            ImportDescription::Function(0),
        );
        module.add_import(import.clone());

        let export = Export::new("foobar".into(), ExportDescription::Function(0));
        module.add_export(export.clone());

        let start = Start::new(0);
        module.set_start(Some(start));

        let global = Global::new(
            GlobalType::immutable(IntegerType::I64.into()),
            Expression::new(vec![Instruction::Numeric(NumericInstruction::I64Constant(
                0,
            ))]),
        );
        module.add_global(global.clone());

        assert_eq!(module.types(), &[function_type]);
        assert_eq!(module.functions(), &[function]);
        assert_eq!(module.tables(), &[table]);
        assert_eq!(module.memories(), &[memory]);
        assert_eq!(module.globals(), &[global]);
        assert_eq!(module.imports(), &[import]);
        assert_eq!(module.exports(), &[export]);
        assert_eq!(module.data(), &[data]);
        assert_eq!(module.elements(), &[element]);
        assert_eq!(module.start(), Some(&start));
    }

    #[test]
    fn new_function() {
        let kind = 1;
        let locals = ResultType::from(vec![IntegerType::I64.into()]);
        let body = Expression::new(Vec::new());
        let function = Function::new(kind, locals.clone(), body.clone());

        assert_eq!(function.kind(), 1);
        assert_eq!(function.locals(), &locals);
        assert_eq!(function.body(), &body);
    }

    #[test]
    fn new_elements() {
        let kind = ReferenceType::Function;
        let mode = ElementMode::Active(0, Expression::new(Vec::new()));
        let initializers = ElementInitializer::FunctionIndex(vec![1]);
        let element = Element::new(kind, mode.clone(), initializers.clone());

        assert_eq!(element.mode(), &mode);
        assert_eq!(element.initializers(), &initializers);
        assert_eq!(element.kind(), &kind);
    }

    #[test]
    fn new_data() {
        let mode = DataMode::Active(0, Expression::new(Vec::new()));
        let initializer = vec![42];
        let data = Data::new(mode.clone(), initializer.clone());

        assert_eq!(data.mode(), &mode);
        assert_eq!(data.initializer(), &initializer);
        assert_eq!(data.len(), initializer.len());
    }

    #[test]
    fn new_table() {
        let kind = TableType::new(Limit::new(0, None), ReferenceType::Function);
        let table = Table::new(kind);

        assert_eq!(table.kind(), &kind);
    }

    #[test]
    fn new_memory() {
        let kind = MemoryType::from(Limit::new(0, None));
        let memory = Memory::new(kind);

        assert_eq!(memory.kind(), &kind);
    }

    #[test]
    fn new_import() {
        let module = Name::from("test");
        let name = Name::from("foobar");
        let kind = MemoryType::from(Limit::new(0, None));
        let description = ImportDescription::Memory(kind);
        let import = Import::new(module.clone(), name.clone(), description);

        assert_eq!(import.module(), &module);
        assert_eq!(import.name(), &name);
        assert_eq!(import.description(), &description);
    }

    #[test]
    fn new_export() {
        let name = Name::from("foobar");
        let description = ExportDescription::Function(42);
        let export = Export::new(name.clone(), description);

        assert_eq!(export.name(), &name);
        assert_eq!(export.description(), &description);
    }

    #[test]
    fn new_start() {
        let function = 42;
        let start = Start::new(function);

        assert_eq!(start.function(), function);
    }

    #[test]
    fn new_global() {
        let kind = GlobalType::mutable(IntegerType::I64.into());
        let expression = Expression::new(Vec::new());
        let global = Global::new(kind, expression.clone());

        assert_eq!(global.kind(), &kind);
        assert_eq!(global.initializer(), &expression);
    }
}
