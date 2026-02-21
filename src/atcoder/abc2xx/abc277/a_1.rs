// https://atcoder.jp/contests/abc277/tasks/abc277_a

fn run(_n: usize, x: usize, p: Vec<usize>) -> usize {
    p.into_iter()
        .position(|num| num == x)
        .unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc277_a() {
        let tests = [
            TestCase(4, 3, vec![2, 3, 1, 4], 2),
            TestCase(5, 2, vec![3, 5, 1, 4, 2], 5),
            TestCase(6, 6, vec![1, 2, 3, 4, 5, 6], 6),
        ];

        for TestCase(n, x, p, expected) in tests {
            assert_eq!(run(n, x, p), expected);
        }
    }
}
