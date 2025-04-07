// https://atcoder.jp/contests/abc055/tasks/abc055_a

fn run(n: usize) -> usize {
    let pay = n * 800;
    let back = (n / 15) * 200;

    pay - back
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(20, 15800),
            TestCase(60, 47200),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
