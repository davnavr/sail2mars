#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("the entry point function was not defined")]
    MissingEntryPointFunction,
    #[error(transparent)]
    LoaderError(#[from] sailar_get::loader::Error),
    #[error(transparent)]
    InputOutputError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
