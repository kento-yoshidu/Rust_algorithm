// https://atcoder.jp/contests/yahoo-procon2019-qual/tasks/yahoo_procon2019_qual_a

fn run(n: usize, k: usize) -> &'static str {
    if n >= k * 2 - 1 {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, "YES"),
            TestCase(5, 5, "NO"),
            TestCase(31, 10, "YES"),
            TestCase(10, 90, "NO"),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
