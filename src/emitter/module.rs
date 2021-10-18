use crate::compiler::emitter::instruction::emit_expression;
use crate::compiler::emitter::{
    emit_byte, emit_bytes, emit_global_type, emit_i32, emit_memory_type, emit_name,
    emit_reference_type, emit_table_type, emit_u32, emit_usize, emit_value_type, emit_vector,
    CountingWrite,
};
use crate::compiler::errors::CompilerError;
use crate::syntax::web_assembly::{
    Data, DataMode, Element, ElementInitializer, ElementMode, Export, ExportDescription, Function,
    Global, Import, ImportDescription, Memory, Name, ReferenceType, Start, Table,
};
use std::io::Write;

/// Emit a function to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#function-section
pub fn emit_function<O: Write + ?Sized>(
    function: &Function,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut counter = CountingWrite::new();
    let mut bytes = 0;

    emit_function_code(function, &mut counter)?;

    bytes += emit_usize(counter.bytes(), output)?;
    bytes += emit_function_code(function, output)?;

    Ok(bytes)
}

/// Emits the code (local types and body) portion of a function.
fn emit_function_code<O: Write + ?Sized>(
    function: &Function,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_usize(function.locals().len(), output)?;

    for local in function.locals().kinds() {
        emit_u32(1u32, output)?;
        emit_value_type(local, output)?;
    }

    emit_expression(function.body(), output)
}

/// Emit an import to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#import-section
pub fn emit_import<O: Write + ?Sized>(
    import: &Import,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += emit_name(import.module(), output)?;
    bytes += emit_name(import.name(), output)?;
    bytes += emit_import_description(import.description(), output)?;

    Ok(bytes)
}

/// Emit an import description to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#import-section
pub fn emit_import_description<O: Write + ?Sized>(
    description: &ImportDescription,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    match description {
        ImportDescription::Function(index) => {
            bytes += emit_byte(0x00u8, output)?;
            bytes += emit_usize(index, output)?;
        }
        ImportDescription::Table(table_type) => {
            bytes += emit_byte(0x01u8, output)?;
            bytes += emit_table_type(table_type, output)?;
        }
        ImportDescription::Memory(memory_type) => {
            bytes += emit_byte(0x02u8, output)?;
            bytes += emit_memory_type(memory_type, output)?;
        }
        ImportDescription::Global(global_type) => {
            bytes += emit_byte(0x03u8, output)?;
            bytes += emit_global_type(global_type, output)?;
        }
    };

    Ok(bytes)
}

/// Emit a table to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#table-section
pub fn emit_table<O: Write + ?Sized>(
    table: &Table,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_table_type(table.kind(), output)
}

/// Emit a memory to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#memory-section
pub fn emit_memory<O: Write + ?Sized>(
    memory: &Memory,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_memory_type(memory.kind(), output)
}

/// Emit a global to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#global-section
pub fn emit_global<O: Write + ?Sized>(
    global: &Global,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += emit_global_type(global.kind(), output)?;
    bytes += emit_expression(global.initializer(), output)?;

    Ok(bytes)
}

/// Emit an export to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#export-section
pub fn emit_export<O: Write + ?Sized>(
    export: &Export,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += emit_name(export.name(), output)?;
    bytes += emit_export_description(export.description(), output)?;

    Ok(bytes)
}

/// Emit an export description to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#export-section
pub fn emit_export_description<O: Write + ?Sized>(
    description: &ExportDescription,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let (value, index) = match description {
        ExportDescription::Function(index) => (0x00, index),
        ExportDescription::Table(index) => (0x01, index),
        ExportDescription::Memory(index) => (0x02, index),
        ExportDescription::Global(index) => (0x03, index),
    };
    let mut bytes = 0;

    bytes += emit_i32(value, output)?;
    bytes += emit_usize(index, output)?;

    Ok(bytes)
}

/// Emit a start to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#start-section
pub fn emit_start<O: Write + ?Sized>(
    start: &Start,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_usize(start.function(), output)
}

/// Emit an element to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#element-section
pub fn emit_element<O: Write + ?Sized>(
    element: &Element,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    match (element.initializers(), element.mode(), element.kind()) {
        (
            ElementInitializer::FunctionIndex(indices),
            ElementMode::Active(0, offset),
            ReferenceType::Function,
        ) => {
            bytes += emit_byte(0x00u8, output)?;
            bytes += emit_expression(offset, output)?;
            bytes += emit_vector(indices, output, emit_usize)?;
        }
        (
            ElementInitializer::FunctionIndex(indices),
            ElementMode::Passive,
            ReferenceType::Function,
        ) => {
            bytes += emit_byte(0x01u8, output)?;
            bytes += emit_byte(0x00u8, output)?;
            bytes += emit_vector(indices, output, emit_usize)?;
        }
        (ElementInitializer::FunctionIndex(indices), ElementMode::Active(table, offset), kind) => {
            bytes += emit_byte(0x02u8, output)?;
            bytes += emit_usize(table, output)?;
            bytes += emit_expression(offset, output)?;
            bytes += emit_reference_type(kind, output)?;
            bytes += emit_vector(indices, output, emit_usize)?;
        }
        (ElementInitializer::FunctionIndex(indices), ElementMode::Declarative, kind) => {
            bytes += emit_byte(0x03u8, output)?;
            bytes += emit_reference_type(kind, output)?;
            bytes += emit_vector(indices, output, emit_usize)?;
        }
        (
            ElementInitializer::Expression(expressions),
            ElementMode::Active(0, offset),
            ReferenceType::Function,
        ) => {
            bytes += emit_byte(0x04u8, output)?;
            bytes += emit_expression(offset, output)?;
            bytes += emit_vector(expressions, output, emit_expression)?;
        }
        (ElementInitializer::Expression(expressions), ElementMode::Passive, kind) => {
            bytes += emit_byte(0x05u8, output)?;
            bytes += emit_reference_type(kind, output)?;
            bytes += emit_vector(expressions, output, emit_expression)?;
        }
        (ElementInitializer::Expression(expressions), ElementMode::Active(table, offset), kind) => {
            bytes += emit_byte(0x06u8, output)?;
            bytes += emit_usize(table, output)?;
            bytes += emit_expression(offset, output)?;
            bytes += emit_reference_type(kind, output)?;
            bytes += emit_vector(expressions, output, emit_expression)?;
        }
        (ElementInitializer::Expression(expressions), ElementMode::Declarative, kind) => {
            bytes += emit_byte(0x07u8, output)?;
            bytes += emit_reference_type(kind, output)?;
            bytes += emit_vector(expressions, output, emit_expression)?;
        }
        _ => return Err(CompilerError::InvalidSyntax),
    };

    Ok(bytes)
}

/// Emit a data to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#data-section
pub fn emit_data<O: Write + ?Sized>(data: &Data, output: &mut O) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    match data.mode() {
        DataMode::Active(0, offset) => {
            bytes += emit_byte(0x00u8, output)?;
            bytes += emit_expression(offset, output)?;
        }
        DataMode::Passive => {
            bytes += emit_byte(0x01u8, output)?;
        }
        DataMode::Active(memory, offset) => {
            bytes += emit_byte(0x02u8, output)?;
            bytes += emit_usize(memory, output)?;
            bytes += emit_expression(offset, output)?;
        }
    };

    bytes += emit_bytes(data.initializer(), output, true)?;

    Ok(bytes)
}

/// Emit named custom content to the module.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#custom-section
pub fn emit_custom_content<O: Write + ?Sized>(
    name: &Name,
    content: &[u8],
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += emit_name(name, output)?;
    bytes += emit_bytes(content, output, false)?;

    Ok(bytes)
}
