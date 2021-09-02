use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[cfg(feature = "text")]
    #[error(
        "The WebAssembly module in text format could not be transformed to the binary format."
    )]
    InvalidText(#[from] wat::Error),
}
