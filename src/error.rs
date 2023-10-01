#[derive(Debug, Clone, Copy)]
pub enum Error {
    KeyAlreadyExists,
    KeyWasNotFound,
    UnexpectedError,
    FileAlreadyExists,
    FileAlreadyOpened,
    FileDoesntExist,
    DirAlreadyExists,
    DirDoesntExist,
    PathDoesntExist,
    ErrorDeserializing,
    ErrorSerializing,
    OutOfBounds,
}

impl std::convert::From<std::io::Error> for Error {
    fn from(_e: std::io::Error) -> Error {
        Error::UnexpectedError
    }
}
