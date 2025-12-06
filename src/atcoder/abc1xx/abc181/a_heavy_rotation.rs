// https://atcoder.jp/contests/abc181/tasks/abc181_a

fn run(n: usize) -> &'static str {
    if n % 2 == 0 {
        "White"
    } else {
        "Black"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc181_a() {
        let tests = [
            TestCase(2, "White"),
            TestCase(5, "Black"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
