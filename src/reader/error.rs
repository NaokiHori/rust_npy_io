#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidUTF8(std::str::Utf8Error),
    MissingKeyValuePairs(String),
    MultipleKeyValuePairs(String),
    InvalidBoolFoundInString(String),
    ParseInt(std::num::ParseIntError),
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ParseError::InvalidUTF8(error) => {
                write!(f, "Invalid utf-8 characters are found: '{}'", error)
            }
            ParseError::MissingKeyValuePairs(key) => {
                write!(f, "Key-value pair is missing: '{}'", key)
            }
            ParseError::MultipleKeyValuePairs(key) => {
                write!(f, "Multiple key-value pairs are found: '{}'", key)
            }
            ParseError::InvalidBoolFoundInString(invalid_value) => {
                write!(f, "Invalid boolean found in string: '{}'", invalid_value)
            }
            ParseError::ParseInt(error) => {
                write!(f, "Invalid integer found in string: {}", error)
            }
        }
    }
}

impl ParseError {
    pub fn missing_key_value_pairs(key: &str) -> Self {
        ParseError::MissingKeyValuePairs(key.to_string())
    }

    pub fn multiple_key_value_pairs(key: &str) -> Self {
        ParseError::MultipleKeyValuePairs(key.to_string())
    }

    pub fn invalid_bool_found_in_string(invalid_value: &str) -> Self {
        ParseError::InvalidBoolFoundInString(invalid_value.to_string())
    }
}
