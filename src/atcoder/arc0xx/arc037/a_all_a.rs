// https://atcoder.jp/contests/arc037/tasks/arc037_a

fn run(_n: usize, m: Vec<usize>) -> usize {
    m.into_iter()
        .filter(|num| *num < 80)
        .map(|num| 80 - num)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![70, 90, 60, 80, 50], 60),
            TestCase(6, vec![100, 100, 100, 100, 100, 100], 0),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
