// https://atcoder.jp/contests/abc379/tasks/abc379_a

fn run(n: &str) -> String {
    let chars: Vec<char> = n.chars().collect();

    format!("{}{}{} {}{}{}", chars[1], chars[2], chars[0], chars[2], chars[0], chars[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("379", "793 937"),
            TestCase("919", "199 991"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
