// https://atcoder.jp/contests/abc439/tasks/abc439_a

fn run(n: i32) -> i32 {
    2_i32.pow(n as u32) - 2 * n
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, i32);

    #[test]
    fn abc439_a() {
        let tests = [
            TestCase(1, 0),
            TestCase(2, 0),
            TestCase(11, 2026),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
