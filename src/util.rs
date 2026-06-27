pub fn digits(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii_digit()).collect()
}
