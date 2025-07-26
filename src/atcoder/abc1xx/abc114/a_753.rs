// https://atcoder.jp/contests/abc114/tasks/abc114_a

fn run(x: usize) -> &'static str {
    if x == 3 || x == 5 || x == 7 {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc114_a() {
        let tests = [
            TestCase(5, "YES"),
            TestCase(6, "NO"),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
