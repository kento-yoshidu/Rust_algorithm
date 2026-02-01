// https://atcoder.jp/contests/abc245/tasks/abc245_a

fn run(a: usize, b: usize, c: usize, d: usize) -> &'static str {
    if a*60 + b <= c*60 + d {
        "Takahashi"
    } else {
        "Aoki"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, &'static str);

    #[test]
    fn abc245_a() {
        let tests = [
            TestCase(7, 0, 6, 30, "Aoki"),
            TestCase(7, 30, 7, 30, "Takahashi"),
            TestCase(0, 0, 23, 59, "Takahashi"),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
