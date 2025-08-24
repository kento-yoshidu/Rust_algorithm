// https://atcoder.jp/contests/abc132/tasks/abc132_b

fn run(_n: usize, p: Vec<usize>) -> usize {
    p.windows(3)
        .filter(|v| {
            (v[0] < v[1] && v[1] < v[2]) || (v[0] > v[1] && v[1] > v[2])
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc132_b() {
        let tests = [
            TestCase(5, vec![1, 3, 5, 4, 2], 2),
            TestCase(9, vec![9, 6, 3, 2, 5, 8, 7, 4, 1], 5),
        ];

        for TestCase(n, p, expected) in tests {
            assert_eq!(run(n, p), expected);
        }
    }
}
