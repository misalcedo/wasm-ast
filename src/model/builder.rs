use crate::model::{
    Data, Element, Export, Function, FunctionType, Global, Import, Memory, Module, Start, Table,
};

pub struct Indexed<T> {
    index: u128,
    value: T,
}

impl<T> Indexed<T> {
    pub fn value(self) -> T {
        self.value
    }
}

pub struct ModuleBuilder {
    function_types: Vec<Indexed<FunctionType>>,
    functions: Vec<Indexed<Function>>,
    tables: Vec<Indexed<Table>>,
    memories: Vec<Indexed<Memory>>,
    globals: Vec<Indexed<Global>>,
    elements: Vec<Indexed<Element>>,
    data: Vec<Indexed<Data>>,
    start: Option<Start>,
    imports: Vec<Indexed<Import>>,
    exports: Vec<Export>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        ModuleBuilder {
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

    pub fn build(self) -> Module {
        Module::new()
    }
}
