pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return Vec::new();
    }

    let s = digits.chars().collect::<Vec<_>>();

    if len == 0 {
        return vec![String::new(); s.len() + 1];
    }

    s.windows(len)
        .map(|w| w.iter().collect::<String>())
        .collect()
}
