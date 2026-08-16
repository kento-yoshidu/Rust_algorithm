// https://atcoder.jp/contests/awc0001/tasks/awc0001_a

fn run(k: usize) -> usize {
    k + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn awc0001_a() {
        let tests = [
            TestCase(2, 3),
            TestCase(10, 11),
            TestCase(1000000000000000000, 1000000000000000001),
        ];

        for TestCase(k, expected) in tests {
            assert_eq!(run(k), expected);
        }
    }
}
