use crate::consts::ENDIAN_SPECIFIERS;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    EmptyDescr,
    UnquotedDescr(String),
    NoEndianSpecifier(String),
    UnexpectedEndianSpecifier(String),
    NonPositiveShape(Vec<usize>),
}

impl std::error::Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ValidationError::EmptyDescr => {
                write!(f, "Value of 'descr' is empty")
            }
            ValidationError::UnquotedDescr(value) => {
                write!(f, "Value of 'descr' is not quoted: {}", value)
            }
            ValidationError::NoEndianSpecifier(value) => {
                write!(f, "Endian is not specified: {}", value)
            }
            ValidationError::UnexpectedEndianSpecifier(value) => {
                write!(
                    f,
                    "An endian specifier other than [{}] is found: {}",
                    ENDIAN_SPECIFIERS
                        .iter()
                        .map(|&c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                    value
                )
            }
            ValidationError::NonPositiveShape(vector) => {
                write!(f, "Non-positive item is found in shape: {:?}", vector)
            }
        }
    }
}

impl ValidationError {
    pub fn unquoted_descr(value: &str) -> Self {
        ValidationError::UnquotedDescr(value.to_string())
    }

    pub fn no_endian_specifier(value: &str) -> Self {
        ValidationError::NoEndianSpecifier(value.to_string())
    }

    pub fn unexpected_endian_specifier(value: &str) -> Self {
        ValidationError::UnexpectedEndianSpecifier(value.to_string())
    }
}
