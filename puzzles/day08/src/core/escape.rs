pub fn escape(s: &str) -> anyhow::Result<String> {
    let mut chars = vec!['"'];

    for c in s.chars() {
        match c {
            '\\' | '"' => {
                chars.push('\\');
                chars.push(c);
            }
            _ => {
                chars.push(c);
            }
        }
    }

    chars.push('"');

    Ok(chars.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescape() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(escape($input).unwrap(), $expected);
            };
        }

        test!("\"\"", "\"\\\"\\\"\"");
        test!("\"abc\"", "\"\\\"abc\\\"\"");
        test!("\"aaa\\\"aaa\"", "\"\\\"aaa\\\\\\\"aaa\\\"\"");
        test!("\"\\x27\"", "\"\\\"\\\\x27\\\"\"");
    }
}
