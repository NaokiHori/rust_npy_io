use crate::consts::ENDIAN_SPECIFIERS;
use crate::writer::error::ValidationError;

/// Checks if the given string is a proper 'descr' value.
///
/// The 'descr' value is limited for simple data types (e.g., "'<f8'") for now.
/// The value of 'descr' key requests the following things.
///
/// - It is a Pythonic string: namely the value is singly or doubly-quoted.
/// - The first character after a quotation is an endian specifier.
pub fn prepare_descr(descr: &str) -> Result<String, ValidationError> {
    // reject empty string
    if descr.is_empty() {
        return Err(ValidationError::EmptyDescr);
    }
    // check if the descr value is doubly / singly quoted
    {
        let is_quoted = |q: char, s: &str| s.starts_with(q) && s.ends_with(q);
        if !is_quoted('"', descr) && !is_quoted('\'', descr) {
            return Err(ValidationError::unquoted_descr(descr));
        }
    }
    // check the descr value is preceded by a proper endian specifier
    {
        let second_character: char = match descr.chars().nth(1) {
            Some(character) => character,
            None => {
                return Err(ValidationError::no_endian_specifier(descr));
            }
        };
        let is_expected_second_char = ENDIAN_SPECIFIERS
            .iter()
            .any(|&endian_specifier: &char| second_character == endian_specifier);
        if !is_expected_second_char {
            return Err(ValidationError::unexpected_endian_specifier(descr));
        }
    }
    Ok(descr.to_string())
}

pub fn prepare_fortran_order(fortran_order: bool) -> Result<String, ValidationError> {
    let fortran_order = if fortran_order {
        "True".to_string()
    } else {
        "False".to_string()
    };
    Ok(fortran_order)
}

pub fn prepare_shape(shape: &[usize]) -> Result<String, ValidationError> {
    if shape.iter().any(|&elem| elem == 0usize) {
        return Err(ValidationError::NonPositiveShape(shape.to_vec()));
    }
    let shape = shape
        .iter()
        .map(|&value: &usize| value.to_string())
        .collect::<Vec<String>>()
        .join(",");
    Ok(format!("({})", shape))
}

#[cfg(test)]
mod tests {
    use super::{prepare_descr, prepare_fortran_order, prepare_shape};
    use crate::writer::error::ValidationError;

    #[test]
    fn prepare_descr_normal() {
        assert_eq!(prepare_descr(r#"'<i8'"#), Ok(r#"'<i8'"#.to_string()));
        assert_eq!(prepare_descr(r#""<i8""#), Ok(r#""<i8""#.to_string()));
    }

    #[test]
    fn prepare_fortran_order_normal() {
        assert_eq!(prepare_fortran_order(true), Ok(r"True".to_string()));
        assert_eq!(prepare_fortran_order(false), Ok(r"False".to_string()));
    }

    #[test]
    fn prepare_shape_normal() {
        assert_eq!(prepare_shape(&[]), Ok(r"()".to_string()));
        assert_eq!(prepare_shape(&[1usize]), Ok(r"(1)".to_string()));
        assert_eq!(prepare_shape(&[1usize, 2usize]), Ok(r"(1,2)".to_string()));
    }

    #[test]
    fn prepare_descr_corner() {
        assert_eq!(prepare_descr(r#""#), Err(ValidationError::EmptyDescr));
        assert_eq!(
            prepare_descr(r#"<i8"#),
            Err(ValidationError::unquoted_descr("<i8"))
        );
        let patterns = [r#"'"#, r#"""#];
        for pattern in patterns.iter() {
            assert_eq!(
                prepare_descr(pattern),
                Err(ValidationError::no_endian_specifier(pattern))
            );
        }
        let patterns = [r#"'hoge'"#, r#""hoge""#, r#"'''"#, r#"""""#];
        for pattern in patterns.iter() {
            assert_eq!(
                prepare_descr(pattern),
                Err(ValidationError::unexpected_endian_specifier(pattern))
            );
        }
    }

    #[test]
    fn prepare_shape_corner() {
        let buf = [0usize];
        assert_eq!(
            prepare_shape(&buf),
            Err(ValidationError::NonPositiveShape(buf.to_vec()))
        );
        let buf = [1usize, 0usize, 2usize];
        assert_eq!(
            prepare_shape(&buf),
            Err(ValidationError::NonPositiveShape(buf.to_vec()))
        );
    }
}
