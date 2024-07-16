// https://atcoder.jp/contests/arc131/tasks/arc131_a

pub fn run(a: usize, b: usize) -> usize {
    500000000 * b + a
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(13, 62, 31000000013),
            TestCase(69120, 824, 412000069120),
            TestCase(6283185, 12566370, 6283185006283185),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
