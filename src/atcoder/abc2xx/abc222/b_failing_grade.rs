// https://atcoder.jp/contests/abc320/tasks/abc320_a

fn run(_n: usize, p: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .filter(|num| *num < p )
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc222_b() {
        let tests = [
            TestCase(4, 50, vec![80, 60, 40, 0], 2),
            TestCase(3, 90, vec![89, 89, 89], 3),
            TestCase(2, 22, vec![6, 37], 1),
        ];

        for TestCase(n, p, a, expected) in tests {
            assert_eq!(run(n, p, a), expected);
        }
    }
}
