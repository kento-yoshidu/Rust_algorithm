// https://atcoder.jp/contests/joi2022yo1c/tasks/joi2022_yo1c_a

fn run(a: usize, b: usize) -> usize {
    b - a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(150, 155, 5),
            TestCase(100, 101, 1),
            TestCase(100, 200, 100),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
