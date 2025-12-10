// https://atcoder.jp/contests/abc190/tasks/abc190_a

fn run(a: usize, b: usize, c: usize) -> &'static str {
    if c == 0 {
        if a > b {
            "Takahashi"
        } else {
            "Aoki"
        }
    } else {
        if a >= b {
            "Takahashi"
        } else {
            "Aoki"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn abc190_a() {
        let tests = [
            TestCase(2, 1, 0, "Takahashi"),
            TestCase(2, 2, 0, "Aoki"),
            TestCase(2, 2, 1, "Takahashi"),
            TestCase(80, 81, 1, "Aoki"),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
