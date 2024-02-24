// https://atcoder.jp/contests/arc108/tasks/arc108_a

fn run(s: usize, p: usize) -> &'static str {
    for i in 1..=p {
        if i*i > p {
            break;
        }

        if p % i == 0 && i + p/i == s {
            return "Yes"
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, "Yes"),
            TestCase(1000000000000, 1, "No"),
            TestCase(3, 2, "Yes"),
            TestCase(2, 1, "Yes"),
        ];

        for TestCase(s, p, expected) in tests {
            assert_eq!(run(s, p), expected);
        }
    }
}
