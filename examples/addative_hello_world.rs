use wasm_ast::{
    ControlInstruction, Custom, Data, Export, Expression, Function, FunctionType, Import, Limit,
    Memory, Module, ModuleSection, ResultType, Start, ValueType,
};

fn main() {
    let message = "Hello, World!";
    let mut builder = Module::builder();

    let print_type = builder.add_function_type(FunctionType::side_effect(
        vec![ValueType::I32, ValueType::I32].into(),
    ));
    let print_function =
        builder.add_import(Import::function("console".into(), "log".into(), print_type));

    let start_type = builder.add_function_type(FunctionType::runnable());
    let start_function = builder.add_function(Function::new(
        start_type,
        ResultType::empty(),
        vec![
            0i32.into(),
            message.len().into(),
            ControlInstruction::Call(print_function).into(),
        ]
        .into(),
    ));
    builder.set_start(Start::new(start_function));
    let memory = builder.add_memory(Memory::new(Limit::bounded(1, 4).into()));
    builder.add_export(Export::memory("memory".into(), memory));
    builder.add_data(Data::active(
        memory,
        Expression::empty(),
        Vec::from(message),
    ));
    builder.add_custom_section(
        ModuleSection::Custom,
        Custom::new("version".into(), Vec::from("1.0.0")),
    );
    builder.add_custom_section(
        ModuleSection::Export,
        Custom::new("footer".into(), Vec::from("foot")),
    );
    builder.set_include_data_count(true);

    let module = builder.build();

    assert_eq!(
        module
            .custom_sections_at(ModuleSection::Custom)
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        module
            .custom_sections_at(ModuleSection::Export)
            .unwrap()
            .len(),
        1
    );
    assert!(module.start().is_some());
    assert_eq!(module.function_types().unwrap().len(), 2);
    assert_eq!(module.imports().unwrap().len(), 1);
    assert_eq!(module.functions().unwrap().len(), 1);
    assert_eq!(module.memories().unwrap().len(), 1);
    assert_eq!(module.data().unwrap().len(), 1);
    assert_eq!(module.exports().unwrap().len(), 1);
    assert_eq!(module, module.clone());
    assert!(module.include_data_count());
}
