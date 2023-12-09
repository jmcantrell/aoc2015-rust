pub type Sequence<'a> = &'a str;

pub fn look_and_say(s: &str) -> String {
    let mut chars = s.chars().peekable();
    let mut result = Vec::new();

    while let Some(c) = chars.next() {
        let mut count = 1;

        loop {
            if chars.next_if_eq(&c).is_some() {
                count += 1;
            } else {
                break;
            }
        }

        result.push(char::from_u32(48 + count).unwrap());
        result.push(c);
    }

    result.into_iter().collect()
}

pub fn iter_look_and_say(mut s: String) -> impl Iterator<Item = String> {
    std::iter::from_fn(move || {
        s = look_and_say(&s);
        Some(s.clone())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(look_and_say($input), $expected);
            };
        }

        test!("1", "11");
        test!("11", "21");
        test!("21", "1211");
        test!("1211", "111221");
        test!("111221", "312211");
    }

    #[test]
    fn test_iter_look_and_say() {
        assert_eq!(
            iter_look_and_say("1".to_owned()).nth(4),
            Some("312211".to_owned())
        );
    }
}
