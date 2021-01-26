pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    let mut digits_end = s
        .char_indices()
        .find_map(|idx, c| if c.is_ascii_digit() { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];
    (remainder, digits)
}

#[cfs(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn extract_multiple_digit() {
        assert_eq!(extract_digits("121-1323"), ("-1323", "121"));
    }
}
