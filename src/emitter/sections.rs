use crate::about;
use crate::compiler::emitter::module::{
    emit_custom_content, emit_data, emit_element, emit_export, emit_function, emit_global,
    emit_import, emit_memory, emit_start, emit_table,
};
use crate::compiler::emitter::{
    emit_byte, emit_bytes, emit_function_type, emit_usize, emit_vector, CountingWrite,
};
use crate::compiler::CompilerError;
use crate::syntax::web_assembly::{Function, Module, Name, TypeIndex};
use std::io::Write;

/// A magic constant used to quickly identify WebAssembly binary file contents.
const PREAMBLE: [u8; 4] = [0x00u8, 0x61u8, 0x73u8, 0x6Du8];

/// The version of the binary WebAssembly format emitted.
const VERSION: [u8; 4] = [0x01u8, 0x00u8, 0x00u8, 0x00u8];

/// Emit a module to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html
pub fn emit_module<O: Write>(module: &Module, output: &mut O) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += emit_bytes(&PREAMBLE, output, false)?;
    bytes += emit_bytes(&VERSION, output, false)?;
    bytes += emit_version_custom_section(output)?;
    bytes += emit_type_section(module, output)?;
    bytes += emit_import_section(module, output)?;
    bytes += emit_function_section(module, output)?;
    bytes += emit_table_section(module, output)?;
    bytes += emit_memory_section(module, output)?;
    bytes += emit_global_section(module, output)?;
    bytes += emit_export_section(module, output)?;
    bytes += emit_start_section(module, output)?;
    bytes += emit_element_section(module, output)?;
    bytes += emit_data_count_section(module, output)?;
    bytes += emit_code_section(module, output)?;
    bytes += emit_data_section(module, output)?;

    Ok(bytes)
}

/// Emits the type section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#type-section
pub fn emit_type_section<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.types().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::Type, output, |o| {
            emit_vector(module.types(), o, emit_function_type)
        })
    }
}

/// Emits the import section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#import-section
pub fn emit_import_section<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.imports().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::Import, output, |o| {
            emit_vector(module.imports(), o, emit_import)
        })
    }
}

/// Emits the function section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#function-section
pub fn emit_function_section<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.functions().is_empty() {
        Ok(0)
    } else {
        let types: Vec<TypeIndex> = module.functions().iter().map(Function::kind).collect();

        emit_section(ModuleSection::Function, output, move |o| {
            emit_vector(types.as_slice(), o, emit_usize)
        })
    }
}

/// Emits the table section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#table-section
pub fn emit_table_section<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.tables().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::Table, output, |o| {
            emit_vector(module.tables(), o, emit_table)
        })
    }
}

/// Emits the memory section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#memory-section
pub fn emit_memory_section<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.memories().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::Memory, output, |o| {
            emit_vector(module.memories(), o, emit_memory)
        })
    }
}

/// Emits the global section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#global-section
pub fn emit_global_section<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.globals().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::Global, output, |o| {
            emit_vector(module.globals(), o, emit_global)
        })
    }
}

/// Emits the export section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#export-section
pub fn emit_export_section<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.exports().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::Export, output, |o| {
            emit_vector(module.exports(), o, emit_export)
        })
    }
}
/// Emits the start section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#start-section
pub fn emit_start_section<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    match module.start() {
        Some(start) => emit_section(ModuleSection::Start, output, |o| emit_start(start, o)),
        None => Ok(0),
    }
}
/// Emits the elements section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#elements-section
pub fn emit_element_section<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.elements().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::Element, output, |o| {
            emit_vector(module.elements(), o, emit_element)
        })
    }
}

/// Emits the data count section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#data-count-section
pub fn emit_data_count_section<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.data().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::DataCount, output, |o| {
            emit_usize(module.data().len(), o)
        })
    }
}

/// Emits the code section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#code-section
pub fn emit_code_section<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.functions().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::Code, output, |o| {
            emit_vector(module.functions(), o, emit_function)
        })
    }
}

/// Emits the data section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#data-section
pub fn emit_data_section<O: Write>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.data().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::Data, output, |o| {
            emit_vector(module.data(), o, emit_data)
        })
    }
}

/// Emit a custom section with the version of the language the module was compiled.
pub fn emit_version_custom_section<O: Write>(output: &mut O) -> Result<usize, CompilerError> {
    emit_section(ModuleSection::Custom, output, |o| {
        let version_section = Name::new("version".to_string());
        emit_custom_content(&version_section, about::VERSION.as_bytes(), o)
    })
}

/// Emits a module section to the given output.
/// Sections need to be prefixed by their length.
/// Since we do not know the length of the emitted contents ahead of time,
/// a buffer is used to hold the emitted values and copy the buffer contents to the output.
/// The buffer is reset before emitting content and after it is copied.
pub fn emit_section<E, O>(
    section: ModuleSection,
    output: &mut O,
    emit: E,
) -> Result<usize, CompilerError>
where
    O: Write,
    E: Fn(&mut dyn Write) -> Result<usize, CompilerError>,
{
    let mut bytes = 0;
    let mut counter = CountingWrite::new();

    emit(&mut counter)?;

    bytes += emit_byte(section as u8, output)?;
    bytes += emit_usize(counter.bytes(), output)?;
    bytes += emit(output)?;

    Ok(bytes)
}

#[derive(Copy, Clone)]
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
