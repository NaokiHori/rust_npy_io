use crate::reader::error::ParseError;

fn find_value(haystack: &str, key: &str, value: &str) -> Result<String, ParseError> {
    let pattern = format!(r#"('{key}'|"{key}")\s*:\s*{value}"#);
    let regex = regex::Regex::new(&pattern).unwrap();
    let pairs: Vec<&str> = regex
        .captures_iter(haystack)
        .map(|c| c.extract())
        .map(|(_, [_key, value])| value)
        .collect();
    match pairs.len() {
        0 => Err(ParseError::missing_key_value_pairs(key)),
        1 => {
            let pair = &pairs[0];
            Ok(pair.to_string())
        }
        _ => Err(ParseError::multiple_key_value_pairs(key)),
    }
}

fn fetch_descr(buf: &str) -> Result<String, ParseError> {
    // TODO: not limited to strings but can accept general dtype.descr
    let descr: String = find_value(buf, "descr", r#"('.+?'|".+?")"#)?;
    Ok(descr.to_string())
}

fn fetch_fortran_order(buf: &str) -> Result<bool, ParseError> {
    let fortran_order: String = find_value(buf, "fortran_order", r#"(True|False)"#)?;
    match fortran_order.as_str() {
        "True" => Ok(true),
        "False" => Ok(false),
        _ => Err(ParseError::invalid_bool_found_in_string(&fortran_order)),
    }
}

fn fetch_shape(buf: &str) -> Result<Vec<usize>, ParseError> {
    let shape: String = find_value(buf, "shape", r#"\(([^)]*)\)"#)?;
    // remove all white spaces for later convenience
    let shape: String = shape
        .chars()
        .filter(|character: &char| !character.is_whitespace())
        .collect::<String>();
    // early return for scalars
    if shape.is_empty() {
        return Ok(Vec::<usize>::new());
    }
    // eliminate trailing comma if any
    let shape: String = {
        let ends_with_comma = shape.ends_with(',');
        if ends_with_comma {
            let length = shape.len();
            shape[0..length - 1].to_string()
        } else {
            shape[..].to_string()
        }
    };
    // finally split with a comma delimiter
    // expect IntErrorKind::Empty for invalid comma use
    let shape: Result<Vec<usize>, std::num::ParseIntError> = shape
        .split(',')
        .map(|s| s.trim().parse::<usize>())
        .collect();
    match shape {
        Ok(shape) => Ok(shape),
        Err(e) => Err(ParseError::ParseInt(e)),
    }
}

pub fn parse(buf: &[u8]) -> Result<crate::Header, ParseError> {
    let buf: &str = std::str::from_utf8(buf)
        .map_err(|error: std::str::Utf8Error| ParseError::InvalidUTF8(error))?;
    let descr: String = fetch_descr(buf)?;
    let fortran_order: bool = fetch_fortran_order(buf)?;
    let shape: Vec<usize> = fetch_shape(buf)?;
    let header = crate::Header {
        descr,
        fortran_order,
        shape,
    };
    Ok(header)
}

#[cfg(test)]
mod tests {
    use super::{fetch_descr, fetch_fortran_order, fetch_shape};
    use crate::reader::error::ParseError;

    #[test]
    fn single_quotations() {
        let haystack = r#"'descr': '<i8', 'fortran_order': False, 'shape': (3, 5), "#;
        assert_eq!(fetch_descr(haystack), Ok(r#"'<i8'"#.to_string()));
        assert_eq!(fetch_fortran_order(haystack), Ok(false));
        assert_eq!(fetch_shape(haystack), Ok([3usize, 5usize].to_vec()));
    }

    #[test]
    fn double_quotations() {
        let haystack = r#""descr": "<i8", "fortran_order": False, "shape": (3, 5), "#;
        assert_eq!(fetch_descr(haystack), Ok(r#""<i8""#.to_string()));
        assert_eq!(fetch_fortran_order(haystack), Ok(false));
        assert_eq!(fetch_shape(haystack), Ok([3usize, 5usize].to_vec()));
    }

    #[test]
    fn inconsistent_quotations() {
        assert_eq!(
            fetch_descr(r#""descr':"<i8""#),
            Err(ParseError::missing_key_value_pairs("descr"))
        );
        assert_eq!(
            fetch_descr(r#"'descr":"<i8""#),
            Err(ParseError::missing_key_value_pairs("descr"))
        );
    }

    #[test]
    fn fetch_descr_normal() {
        assert_eq!(fetch_descr(r#"'descr':'<f8'"#), Ok(r#"'<f8'"#.to_string()));
        assert_eq!(fetch_descr(r#"'descr' :"<f8""#), Ok(r#""<f8""#.to_string()));
        assert_eq!(fetch_descr(r#""descr": '<f8'"#), Ok(r#"'<f8'"#.to_string()));
        assert_eq!(
            fetch_descr(r#""descr" : "<f8""#),
            Ok(r#""<f8""#.to_string())
        );
    }

    #[test]
    fn fetch_fortran_order_normal() {
        assert_eq!(fetch_fortran_order(r#"'fortran_order':True"#), Ok(true));
        assert_eq!(fetch_fortran_order(r#"'fortran_order' :False"#), Ok(false));
        assert_eq!(fetch_fortran_order(r#""fortran_order": True"#), Ok(true));
        assert_eq!(fetch_fortran_order(r#""fortran_order" : False"#), Ok(false));
    }

    #[test]
    fn fetch_shape_normal() {
        assert_eq!(fetch_shape("'shape':()"), Ok(Vec::new()));
        assert_eq!(fetch_shape("'shape':( )"), Ok(Vec::new()));
        assert_eq!(fetch_shape("'shape': (2)"), Ok(vec![2]));
        assert_eq!(fetch_shape("'shape' :(2,)"), Ok(vec![2]));
        assert_eq!(fetch_shape("'shape': (42)"), Ok(vec![42]));
        assert_eq!(fetch_shape("'shape' :(42,)"), Ok(vec![42]));
        assert_eq!(fetch_shape("'shape' : (1, 2,  3)"), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn fetch_descr_corner() {
        assert_eq!(
            fetch_descr("'descr':<i8"),
            Err(ParseError::missing_key_value_pairs("descr"))
        );
        assert_eq!(
            fetch_descr("'descr':,"),
            Err(ParseError::missing_key_value_pairs("descr"))
        );
        assert_eq!(
            fetch_descr("'descr': ,"),
            Err(ParseError::missing_key_value_pairs("descr"))
        );
        assert_eq!(
            fetch_descr(r#"descr:"<i8""#),
            Err(ParseError::missing_key_value_pairs("descr"))
        );
        assert_eq!(
            fetch_descr(r#":"<i8""#),
            Err(ParseError::missing_key_value_pairs("descr"))
        );
        assert_eq!(
            fetch_descr("'descr': '<f8', 'descr': '<i8'"),
            Err(ParseError::multiple_key_value_pairs("descr"))
        );
    }

    #[test]
    fn fetch_fortran_order_corner() {
        assert_eq!(
            fetch_fortran_order(r#""fortran_order':True"#),
            Err(ParseError::missing_key_value_pairs("fortran_order"))
        );
        assert_eq!(
            fetch_fortran_order(r#"'fortran_order":False"#),
            Err(ParseError::missing_key_value_pairs("fortran_order"))
        );
        assert_eq!(
            fetch_fortran_order(r#"'fortran_order':"True""#),
            Err(ParseError::missing_key_value_pairs("fortran_order"))
        );
        assert_eq!(
            fetch_fortran_order("'fortran_order':,"),
            Err(ParseError::missing_key_value_pairs("fortran_order"))
        );
        assert_eq!(
            fetch_fortran_order("'fortran_order': ,"),
            Err(ParseError::missing_key_value_pairs("fortran_order"))
        );
        assert_eq!(
            fetch_fortran_order(r#"fortran_order:"<i8""#),
            Err(ParseError::missing_key_value_pairs("fortran_order"))
        );
        assert_eq!(
            fetch_fortran_order(r#":False"#),
            Err(ParseError::missing_key_value_pairs("fortran_order"))
        );
        assert_eq!(
            fetch_fortran_order("'fortran_order': True, 'fortran_order': False"),
            Err(ParseError::multiple_key_value_pairs("fortran_order"))
        );
    }

    #[test]
    fn fetch_shape_corner() {
        for input in ["'shape': (not_a_number)", "'shape': (1, two, 3)"] {
            let shape = fetch_shape(input);
            assert!(shape.is_err());
            if let Err(ParseError::ParseInt(e)) = shape {
                assert_eq!(e.kind(), &std::num::IntErrorKind::InvalidDigit);
            } else {
                panic!("unreachable");
            }
        }
        for input in ["'shape': (,)", "'shape': (,,)", "'shape': (0,,2)"] {
            let shape = fetch_shape(input);
            assert!(shape.is_err());
            if let Err(ParseError::ParseInt(e)) = shape {
                assert_eq!(e.kind(), &std::num::IntErrorKind::Empty);
            } else {
                panic!("unreachable");
            }
        }
    }
}
