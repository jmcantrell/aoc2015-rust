pub fn ridiculous_is_nice(s: &str) -> bool {
    fn is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
    }

    fn contains_at_least_three_vowels(chars: &[char]) -> bool {
        chars.iter().filter(|&&c| is_vowel(c)).take(3).count() == 3
    }

    fn contains_consecutive_twins(chars: &[char]) -> bool {
        chars.windows(2).any(|pair| pair[0] == pair[1])
    }

    fn contains_bad_pairs(chars: &[char]) -> bool {
        chars.windows(2).any(|pair| {
            pair == ['a', 'b'] || pair == ['c', 'd'] || pair == ['p', 'q'] || pair == ['x', 'y']
        })
    }

    let chars: Vec<_> = s.chars().collect();

    !contains_bad_pairs(&chars)
        && contains_at_least_three_vowels(&chars)
        && contains_consecutive_twins(&chars)
}

pub fn better_is_nice(s: &str) -> bool {
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
pub mod tests {
    #[test]
    fn ridiculous_is_nice() {
        macro_rules! assert_nice {
            ($input:expr) => {
                assert!(super::ridiculous_is_nice($input));
            };
        }

        macro_rules! assert_naughty {
            ($input:expr) => {
                assert!(!super::ridiculous_is_nice($input));
            };
        }

        assert_nice!("ugknbfddgicrmopn");
        assert_nice!("aaa");
        assert_naughty!("jchzalrnumimnmhp");
        assert_naughty!("haegwjzuvuyypxyu");
        assert_naughty!("dvszwmarrgswjxmb");
    }

    #[test]
    fn better_is_nice() {
        macro_rules! assert_nice {
            ($input:expr) => {
                assert!(super::better_is_nice($input));
            };
        }

        macro_rules! assert_naughty {
            ($input:expr) => {
                assert!(!super::better_is_nice($input));
            };
        }

        assert_nice!("qjhvhtzxzqqjkmpb");
        assert_nice!("xxyxx");
        assert_naughty!("uurcxstgmygtbstg");
        assert_naughty!("ieodomkazucvgmuy");
    }
}
