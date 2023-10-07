pub fn is_nice(s: &str) -> bool {
    fn contains_non_overlapping_identical_pairs(chars: &[char]) -> bool {
        if chars.len() < 4 {
            return false;
        }

        for i in 0..(chars.len() - 3) {
            let a = &chars[i..i + 2];
            for j in (i + 2)..(chars.len() - 1) {
                let b = &chars[j..j + 2];
                if a == b {
                    return true;
                }
            }
        }

        false
    }

    fn contains_twins_separated_by_one_char(chars: &[char]) -> bool {
        if chars.len() < 3 {
            return false;
        }

        for i in 0..(chars.len() - 2) {
            let j = i + 2;
            let a = chars[i];
            let b = chars[j];
            if a == b {
                return true;
            }
        }

        false
    }

    let chars: Vec<_> = s.chars().collect();

    contains_non_overlapping_identical_pairs(&chars) && contains_twins_separated_by_one_char(&chars)
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_nice() {
        macro_rules! assert_nice {
            ($input:expr) => {
                assert!(super::is_nice($input));
            };
        }

        macro_rules! assert_naughty {
            ($input:expr) => {
                assert!(!super::is_nice($input));
            };
        }

        assert_nice!("qjhvhtzxzqqjkmpb");
        assert_nice!("xxyxx");
        assert_naughty!("uurcxstgmygtbstg");
        assert_naughty!("ieodomkazucvgmuy");
    }
}
