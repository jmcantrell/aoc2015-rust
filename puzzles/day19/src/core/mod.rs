pub(crate) fn splice(s: &str, i: usize, j: usize, addition: &str) -> String {
    s.split_at(i)
        .0
        .chars()
        .chain(addition.chars())
        .chain(s.split_at(j).1.chars())
        .collect()
}

pub(crate) fn positions<'a>(
    s: &'a str,
    value: &'a str,
) -> impl Iterator<Item = usize> + DoubleEndedIterator + 'a {
    let upper = s
        .len()
        .checked_sub(value.len())
        .map(|value| value + 1)
        .unwrap_or_default();
    (0..upper).filter_map(move |i| (&s[i..i + value.len()] == value).then_some(i))
}
