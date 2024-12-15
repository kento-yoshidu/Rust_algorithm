// https://atcoder.jp/contests/joi2018yo/tasks/joi2018_yo_b

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .fold((0, 0), |(max, acc), x| {
            if x == 1 {
                (max.max(acc+1), acc+1)
            } else {
                (max, 0)
            }
        }).0 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![0, 1, 0, 0, 0], 2),
            TestCase(5, vec![1, 1, 1, 1, 1], 6),
            TestCase(7, vec![0, 0, 1, 0, 1, 1, 0], 3),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
