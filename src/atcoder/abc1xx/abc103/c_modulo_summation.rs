// https://atcoder.jp/contests/abc103/tasks/abc103_c

fn run(_n: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .map(|n| n-1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![3, 4, 6], 10),
            TestCase(5, vec![7, 46, 11, 20, 11], 90),
            TestCase(7, vec![994, 518, 941, 851, 647, 2, 581], 4527),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
