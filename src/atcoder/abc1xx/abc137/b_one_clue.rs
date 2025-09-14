// https://atcoder.jp/contests/abc137/tasks/abc137_b

fn run(k: isize, x: isize) -> Vec<isize> {
    ((x-(k-1))..=(x+(k-1)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, Vec<isize>);

    #[test]
    fn abc137_b() {
        let tests = [
            TestCase(3, 7, vec![5, 6, 7, 8, 9]),
            TestCase(4, 0, vec![-3, -2, -1, 0, 1, 2, 3]),
            TestCase(1, 100, vec![100]),
        ];

        for TestCase(k, x, expected) in tests {
            assert_eq!(run(k, x), expected);
        }
    }
}
