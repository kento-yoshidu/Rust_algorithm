// https://atcoder.jp/contests/abc247/tasks/abc247_c

fn calc(n: usize, s: String) -> String {
    if n == 1 {
        "1".to_string()
    } else {
        format!("{} {} {}", calc(n-1, s.clone()), n.to_string(), calc(n-1, s.clone()))
    }
}

fn run(n: usize) -> String {
    calc(n, String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, String);

    #[test]
    fn abc247_c() {
        let tests = [
            TestCase(2, "1 2 1".to_string()),
            TestCase(1, "1".to_string()),
            TestCase(4, "1 2 1 3 1 2 1 4 1 2 1 3 1 2 1".to_string()),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(expected, run(n));
        }
    }
}
