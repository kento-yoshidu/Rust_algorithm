// https://atcoder.jp/contests/aising2020/tasks/aising2020_b

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .step_by(2)
        .filter(|num| {
            num % 2 != 0
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![1, 3, 4, 5, 7], 2),
            TestCase(15, vec![13, 76, 46, 15, 50, 98, 93, 77, 31, 43, 84, 90, 6, 24, 14], 3),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
