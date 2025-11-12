// https://atcoder.jp/contests/abc204/tasks/abc204_b

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .map(|i| {
            if 10 < i {
                i - 10
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc204_b() {
        let tests = [
            TestCase(3, vec![6, 17, 28], 25),
            TestCase(4, vec![8, 9, 10, 11], 1),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
