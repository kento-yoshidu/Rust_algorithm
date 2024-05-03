// https://atcoder.jp/contests/abc343/tasks/abc343_a

fn run(a: usize, b: usize) -> usize {
    (0..=9)
        .find(|num| {
            a + b != *num
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 5, 0),
            TestCase(0, 0, 1),
            TestCase(7, 4, 0),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
