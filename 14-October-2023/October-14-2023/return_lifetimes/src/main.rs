fn main() {}

fn matches_str(left: &str, right: &str) -> &str {
    if left == right {
        left
    }
}

fn statics<'a>(value: &'a str) -> &'static str {
    value
}
