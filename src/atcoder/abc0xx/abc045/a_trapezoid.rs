// https://atcoder.jp/contests/abc045/tasks/abc045_a

fn run(a: i32, b: i32, h: i32) -> i32 {
    (a + b) * h / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, i32, i32, i32);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, 2, 7),
            TestCase(4, 4, 4, 16),
        ];

        for TestCase(a, b, h, expected) in tests {
            assert_eq!(run(a, b, h), expected)
        }
    }
}
