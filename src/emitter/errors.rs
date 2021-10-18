use thiserror::Error;

/// An error in emitting a WebAssembly module in the binary format.
#[derive(Error, Debug)]
pub enum EmitError {
    #[error("An IO error occurred.")]
    IO(#[from] std::io::Error),
}