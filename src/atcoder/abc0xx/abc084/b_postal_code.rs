// https://atcoder.jp/contests/abc084/tasks/abc084_b

fn run(a: usize, _b: usize, s: &str) -> &'static str {
    let c: Vec<char> = s.chars().collect();

    if c[a] != '-' {
        return "No";
    }

    if c.iter().filter(|&c| {
        *c == '-'
    }).count() != 1 {
        return "No";
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, "269-6650", "Yes"),
            TestCase(1, 1, "---", "No"),
            TestCase(1, 2, "7444", "No"),
        ];

        for TestCase(a, b, s, expected) in tests {
            assert_eq!(run(a, b, s), expected);
        }
    }
}
