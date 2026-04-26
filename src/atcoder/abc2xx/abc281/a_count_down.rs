// https://atcoder.jp/contests/abc281/tasks/abc281_a

fn run(n: usize) -> Vec<usize> {
    (0..=n).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>);

    #[test]
    fn abc281_a() {
        let tests = [
            TestCase(3, vec![3, 2, 1, 0]),
            TestCase(22, vec![22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
