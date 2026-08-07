use std::error::Error as ErrorTrait;
use std::fmt::Display;
use std::string::FromUtf8Error;
use std::io::Error as IoError;

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    StringTooLong,
    StringTooShort,
    StringNotRightLen,
    ContainsNumbers(u32),
    InvalidChunk,
    UTF8Error(FromUtf8Error),
    InvalidHeader(u64),
    ChunkTypeNotFound,
    MalformedInput,
    IncorrectArgs,
    FileDoesNotExist(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StringTooLong => write!(f, "String is too long"),
            Self::StringTooShort => write!(f, "String is too short"),
            Self::StringNotRightLen => write!(f, "idk how you get here, likly memory corruption"),
            Self::ContainsNumbers(chunk_type_data) => write!(
                f,
                "Chunk contains numbers or symbols, data: {:?}",
                chunk_type_data.to_be_bytes()
            ),
            Self::InvalidChunk => write!(f, "Chunk is invalid"),
            Self::UTF8Error(err) => write!(f, "error converting to utf8 string: {err}"),
            Self::InvalidHeader(header) => write!(f, "invalid header: {:?}", header.to_be_bytes()),
            Self::ChunkTypeNotFound => write!(f, "Chunk type not found"),
            Self::MalformedInput => write!(f, "Input cut off or is corrupted in some way"),
            Self::IncorrectArgs => write!(f, "Incorrect args"),
            Self::FileDoesNotExist(error) => write!(f, "Specfied file does not exist: {error}")
        }
    }
}

impl ErrorTrait for Error {}
