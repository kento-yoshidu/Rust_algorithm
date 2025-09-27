// https://atcoder.jp/contests/abc419/tasks/abc419_a

fn run(s: &str) -> &'static str {
    match s {
        "red" => "SSS",
        "blue" => "FFF",
        "green" => "MMM",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc419_a() {
        let tests = [
            TestCase("red", "SSS"),
            TestCase("atcoder", "Unknown"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
