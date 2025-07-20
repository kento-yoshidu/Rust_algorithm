// https://atcoder.jp/contests/abc064/tasks/abc064_b

fn run(_n: usize, a: Vec<usize>) -> usize {
    let min = a.iter().min().unwrap();
    let max = a.iter().max().unwrap();

    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![2, 3, 7, 9], 7),
            TestCase(8, vec![3, 1, 4, 1, 5, 9, 2, 6], 8),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
