// https://atcoder.jp/contests/abc429/tasks/abc429_a

fn run(n: usize, m: usize) -> Vec<&'static str> {
    (0..n)
        .map(|i| {
            if i <= m-1 {
                "OK"
            } else {
                "Too Many Requests"
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>);

    #[test]
    fn abc429_a() {
        let tests = [
            TestCase(5, 3, vec!["OK", "OK", "OK", "Too Many Requests", "Too Many Requests"]),
            TestCase(3, 5, vec!["OK", "OK", "OK"]),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
