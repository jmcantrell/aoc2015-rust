pub type Code = u64;

pub fn iter_codes(start: Code) -> impl Iterator<Item = Code> {
    let mut next = start;

    std::iter::from_fn(move || {
        let current = next;
        next = (current * 252533) % 33554393;
        Some(current)
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn iter_codes() {
        let mut iter = super::iter_codes(20151125);

        macro_rules! assert_next {
            ($expected:expr) => {
                assert_eq!(iter.next(), $expected);
            };
        }

        assert_next!(Some(20151125));
        assert_next!(Some(31916031));
        assert_next!(Some(18749137));
        assert_next!(Some(16080970));
        assert_next!(Some(21629792));
        assert_next!(Some(17289845));
        assert_next!(Some(24592653));
        assert_next!(Some(8057251));
        assert_next!(Some(16929656));
        assert_next!(Some(30943339));
        assert_next!(Some(77061));
        assert_next!(Some(32451966));
        assert_next!(Some(1601130));
        assert_next!(Some(7726640));
        assert_next!(Some(10071777));
        assert_next!(Some(33071741));
        assert_next!(Some(17552253));
        assert_next!(Some(21345942));
        assert_next!(Some(7981243));
        assert_next!(Some(15514188));
        assert_next!(Some(33511524));
    }
}
