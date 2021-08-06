use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tortuga::compiler::emitter::{emit_module, CountingWrite};
use tortuga::model::{
    Data, DataMode, Element, ElementInitializer, ElementMode, Export, ExportDescription,
    Expression, Function, FunctionType, Global, GlobalType, Import, ImportDescription, Instruction,
    Limit, Memory, MemoryType, Module, Name, NumberType, NumericInstruction, ReferenceType,
    ResultType, Start, Table, TableType, ValueType,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("empty", |b| {
        let empty_module = Module::new();

        b.iter(move || {
            let mut write = CountingWrite::new();
            emit_module(&empty_module, &mut write).expect("An error occurred emitting the module.");
            black_box(write.bytes())
        })
    });
    c.bench_function("singular", |b| {
        let singular_module = new_singular_module();

        b.iter(move || {
            let mut write = CountingWrite::new();
            emit_module(&singular_module, &mut write)
                .expect("An error occurred emitting the module.");
            black_box(write.bytes())
        })
    });
}

/// Creates a new module with 1 of each field.
fn new_singular_module() -> Module {
    let mut module = Module::new();
    let function_type = FunctionType::new(
        ResultType::new(vec![ValueType::Number(NumberType::I64)]),
        ResultType::new(vec![ValueType::Number(NumberType::F64)]),
    );
    module.add_type(function_type);

    let function = Function::new(
        0,
        ResultType::new(vec![ValueType::Number(NumberType::I32)]),
        Expression::new(vec![Instruction::Numeric(NumericInstruction::F64Constant(
            0.0,
        ))]),
    );
    module.add_function(function);

    let start_function_type = FunctionType::new(ResultType::new(vec![]), ResultType::new(vec![]));
    module.add_type(start_function_type);

    let import = Import::new(
        Name::new("test".to_string()),
        Name::new("foobar".to_string()),
        ImportDescription::Function(1),
    );
    module.add_import(import);

    let element = Element::new(
        ReferenceType::Function,
        ElementMode::Passive,
        ElementInitializer::FunctionIndex(vec![0]),
    );
    module.add_element(element);

    let data = Data::new(DataMode::Passive, vec![42]);
    module.add_data(data);

    let table = Table::new(TableType::new(Limit::new(1, None), ReferenceType::Function));
    module.add_table(table);

    let memory = Memory::new(MemoryType::new(Limit::new(1, None)));
    module.add_memory(memory);

    let export = Export::new(
        Name::new("foobar".to_string()),
        ExportDescription::Function(0),
    );
    module.add_export(export);

    let start = Start::new(0);
    module.set_start(Some(start));

    let global = Global::new(
        GlobalType::new(false, ValueType::Number(NumberType::I64)),
        Expression::new(vec![Instruction::Numeric(NumericInstruction::I64Constant(
            0,
        ))]),
    );
    module.add_global(global);

    module
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
