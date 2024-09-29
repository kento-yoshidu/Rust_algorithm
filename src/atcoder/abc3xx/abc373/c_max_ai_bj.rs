// https://atcoder.jp/contests/abc373/tasks/abc373_c

fn run(_n: usize, a: Vec<isize>, b: Vec<isize>) -> usize {
    (a.into_iter().max().unwrap() + b.into_iter().max().unwrap()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, Vec<isize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, vec![-1, 5], vec![3, -7], 8),
            TestCase(6, vec![15, 12, 3, -13, -1, -19], vec![7, 17, -13, -10, 18, 4], 33),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
