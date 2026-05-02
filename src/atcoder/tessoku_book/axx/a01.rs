// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_a

fn run(n: usize) -> usize {
    n * n
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn tessoku_a01() {
        let tests = [
            TestCase(2, 4),
            TestCase(8, 64),
            TestCase(100, 10000),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
