pub fn mask_tail(input: &str, visible_prefix: usize, masked_len: usize) -> String {
    let len = input.len();

    if len <= visible_prefix {
        return input.to_string();
    }

    let split = len.saturating_sub(masked_len);
    let (start, end) = input.split_at(split);

    format!("{}{}", start, "*".repeat(end.len()))
}
