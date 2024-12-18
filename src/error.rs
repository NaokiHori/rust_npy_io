#[cfg(feature = "reader")]
use crate::reader::error::ParseError;
#[cfg(feature = "writer")]
use crate::writer::error::ValidationError;

#[cfg(feature = "reader")]
#[derive(Debug)]
pub enum ReadHeaderError {
    Io(std::io::Error),
    InvalidMagicString(Vec<u8>),
    InvalidMajorVersion(u8),
    InvalidMinorVersion(u8),
    InvalidHeaderSize(usize),
    ParseFailed(ParseError),
}

#[cfg(feature = "reader")]
impl std::error::Error for ReadHeaderError {}

#[cfg(feature = "reader")]
impl std::fmt::Display for ReadHeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use crate::consts::HEADER_BLOCK_SIZE;
        match self {
            ReadHeaderError::Io(error) => {
                write!(f, "Io error: {}", error)
            }
            ReadHeaderError::InvalidMagicString(buf) => {
                write!(f, "Invalid magic string is found: '{:?}'", buf)
            }
            ReadHeaderError::InvalidMajorVersion(value) => {
                write!(f, "Invalid major version: '{}'", value)
            }
            ReadHeaderError::InvalidMinorVersion(value) => {
                write!(f, "Invalid minor version: {}", value)
            }
            ReadHeaderError::InvalidHeaderSize(value) => {
                write!(
                    f,
                    "Invalid header size: {}, which should be a multiple of {}",
                    value, HEADER_BLOCK_SIZE
                )
            }
            ReadHeaderError::ParseFailed(error) => {
                write!(f, "Failed to parse dictionary: {}", error)
            }
        }
    }
}

#[cfg(feature = "reader")]
impl From<std::io::Error> for ReadHeaderError {
    fn from(error: std::io::Error) -> Self {
        ReadHeaderError::Io(error)
    }
}

#[cfg(feature = "reader")]
impl From<ParseError> for ReadHeaderError {
    fn from(error: ParseError) -> Self {
        ReadHeaderError::ParseFailed(error)
    }
}

#[cfg(feature = "writer")]
#[derive(Debug)]
pub enum WriteHeaderError {
    Io(std::io::Error),
    ValidationFailed(ValidationError),
    ZeroPaddingSize,
}

#[cfg(feature = "writer")]
impl std::error::Error for WriteHeaderError {}

#[cfg(feature = "writer")]
impl std::fmt::Display for WriteHeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            WriteHeaderError::Io(error) => {
                write!(f, "{}", error)
            }
            WriteHeaderError::ValidationFailed(string) => {
                write!(f, "Illegal argument: {}", string)
            }
            WriteHeaderError::ZeroPaddingSize => {
                write!(f, "Zero padding size is illegal: at least terminating 0x0a is necessary at the end of header")
            }
        }
    }
}

#[cfg(feature = "writer")]
impl From<std::io::Error> for WriteHeaderError {
    fn from(error: std::io::Error) -> Self {
        WriteHeaderError::Io(error)
    }
}

#[cfg(feature = "writer")]
impl From<ValidationError> for WriteHeaderError {
    fn from(error: ValidationError) -> Self {
        WriteHeaderError::ValidationFailed(error)
    }
}
