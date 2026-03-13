// https://atcoder.jp/contests/abc264/tasks/abc264_a

fn run(l: usize, r: usize) -> String {
    "atcoder"[l-1..=r-1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn abc264_a() {
        let tests = [
            TestCase(3, 6, "code"),
            TestCase(4, 4, "o"),
            TestCase(1, 7, "atcoder"),
        ];

        for TestCase(l, r, expected) in tests {
            assert_eq!(run(l, r), expected);
        }
    }
}
