use anyhow::{bail, ensure, Context};

pub fn unescape(s: &str) -> anyhow::Result<String> {
    let mut chars = s.chars().enumerate();
    let (i, c) = chars.next().context("missing opening quote")?;

    ensure!(
        c == '"',
        "expected character number {} to be {:?}, but it was {:?}",
        i + 1,
        '"',
        c
    );

    let mut value = Vec::new();

    while let Some((_, c)) = chars.next() {
        match c {
            '\\' => {
                if let Some((i, c)) = chars.next() {
                    match c {
                        '\\' | '"' => {
                            value.push(c);
                        }
                        'x' => {
                            let digits = s.get(i + 1..i + 3).with_context(|| {
                                format!(
                                    "expected at least two characters after character number {}",
                                    i + 1,
                                )
                            })?;
                            let n = u32::from_str_radix(digits, 16).with_context(|| {
                                format!(
                                    "expected two hexadecimal digits after character number {}",
                                    i + 1,
                                )
                            })?;
                            let c = char::from_u32(n).with_context(|| {
                                format!(
                                    "invalid hexadecimal escape sequence starting at character number {}",
                                    i + 1
                                )
                            })?;
                            chars.next();
                            chars.next();
                            value.push(c);
                        }
                        _ => {
                            bail!("unrecognized escape sequenc identifier: {:?}", c);
                        }
                    }
                } else {
                    bail!(
                        "missing escape sequence identifier after character number {}",
                        i + 1
                    );
                }
            }
            '"' => {
                break;
            }
            _ => {
                value.push(c);
            }
        }
    }

    ensure!(
        chars.next().is_none(),
        "unexpected characters after closing quote"
    );

    Ok(value.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescape() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(unescape($input).unwrap(), $expected);
            };
        }

        test!("\"\"", "");
        test!("\"abc\"", "abc");
        test!("\"aaa\\\"aaa\"", "aaa\"aaa");
        test!("\"\\x27\"", "'");
    }
}
