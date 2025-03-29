/// Convert a string to snake_case.
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            if i != 0
                && !result.ends_with('_')
                && (chars[i - 1].is_lowercase()
                    || (i + 1 < chars.len() && chars[i + 1].is_lowercase()))
            {
                result.push('_');
            }
            result.extend(c.to_lowercase());
        } else if c.is_whitespace() || c == '-' {
            result.push('_');
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("HelloWorld", "hello_world")]
    #[case("helloWorld", "hello_world")]
    #[case("This is A Test", "this_is_a_test")]
    #[case("Test-String", "test_string")]
    #[case("HTTPRequest", "http_request")]
    #[case("", "")]
    fn test_to_snake_case(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(to_snake_case(input), expected);
    }
}
