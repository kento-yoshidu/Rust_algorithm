// https://atcoder.jp/contests/abc245/tasks/abc245_b

fn run(n: usize, a: Vec<usize>) -> usize {
    (0..=n)
        .find(|i| {
            !a.contains(i)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc245_b() {
        let tests = [
            TestCase(8, vec![0, 3, 2, 6, 2, 1, 0, 0], 4),
            TestCase(3, vec![2000, 2000, 2000], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
