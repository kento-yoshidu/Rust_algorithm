// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ey

fn run(n: usize) -> usize {
    (n as f64 * 1.1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn tessoku_c01() {
        let tests = [
            TestCase(800, 880),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
