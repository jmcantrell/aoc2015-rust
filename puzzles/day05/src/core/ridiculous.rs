pub fn is_nice(s: &str) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice() {
        macro_rules! assert_nice {
            ($input:expr) => {
                assert!(is_nice($input));
            };
        }

        macro_rules! assert_naughty {
            ($input:expr) => {
                assert!(!is_nice($input));
            };
        }

        assert_nice!("ugknbfddgicrmopn");
        assert_nice!("aaa");
        assert_naughty!("jchzalrnumimnmhp");
        assert_naughty!("haegwjzuvuyypxyu");
        assert_naughty!("dvszwmarrgswjxmb");
    }
}
