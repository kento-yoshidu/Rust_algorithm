// https://atcoder.jp/contests/arc038/tasks/arc038_a

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut arr = a.clone();

    arr.sort();
    arr.reverse();

    arr.into_iter()
        .step_by(2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, vec![400, 628], 628),
            TestCase(5, vec![2, 5, 9, 6, 5], 16),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
