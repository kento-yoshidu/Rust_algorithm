// https://atcoder.jp/contests/abc320/tasks/abc320_a

fn run(a: u32, b: u32) -> u32 {
    a.pow(b) + b.pow(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u32, u32, u32);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 8, 320),
            TestCase(9, 9, 774840978),
            TestCase(5, 6, 23401),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
