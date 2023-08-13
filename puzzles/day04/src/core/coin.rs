pub type Salt = usize;

pub fn find_salt(secret_key: &[u8], prefix_len: usize) -> Option<Salt> {
    let prefix: String = "0".repeat(prefix_len);
    (0_usize..).find(|n| {
        let mut context = md5::Context::new();
        context.consume(secret_key);
        context.consume(n.to_string());
        format!("{:x}", context.compute()).starts_with(&prefix)
    })
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn find_salt() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    super::find_salt(&$input.as_bytes().to_vec(), 5).unwrap(),
                    $expected
                );
            };
        }

        test!("abcdef", 609043);
        test!("pqrstuv", 1048970);
    }
}
