// https://atcoder.jp/contests/abc366/tasks/abc366_a

fn run(n: usize, t: usize, a: usize) -> &'static str {
    let rest = n - t - a;

    if a > rest+t || t > rest+a {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 4, 2, "Yes"),
            TestCase(99, 12, 48, "No"),
            TestCase(1, 0, 0, "No"),
        ];

        for TestCase(n, t, a, expected) in tests {
            assert_eq!(run(n, t, a), expected);
        }
    }
}
