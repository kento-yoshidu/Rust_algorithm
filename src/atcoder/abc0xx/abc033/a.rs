// https://atcoder.jp/contests/abc033/tasks/abc033_a

fn run(n: usize) -> &'static str {
    if n % 1111 == 0 {
        "SAME"
    } else {
        "DIFFERENT"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc033_a() {
        let tests = [
            TestCase(2222, "SAME"),
            TestCase(1221, "DIFFERENT"),
            TestCase(0000, "SAME"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
