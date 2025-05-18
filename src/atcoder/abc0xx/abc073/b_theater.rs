// https://atcoder.jp/contests/abc073/tasks/abc073_b

fn run(n: usize, l: &Vec<(usize, usize)>) -> usize {
    l.iter().map(|t| {
        t.1 - t.0
    })
    .sum::<usize>() + n
}

fn run2(n: usize, l: &Vec<(usize, usize)>) -> usize {
    l.iter().fold(0, |sum, (l, r)| sum + r - l) + n
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, vec![(24, 30)], 7),
            TestCase(2, vec![(6, 8), (3, 3)], 4),
        ];

        for TestCase(n, l, expected) in tests {
            assert_eq!(run(n, &l), expected);
            assert_eq!(run2(n, &l), expected);
        }
    }
}
