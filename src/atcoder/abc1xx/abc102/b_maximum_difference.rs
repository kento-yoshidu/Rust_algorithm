// https://atcoder.jp/contests/abc102/tasks/abc102_b

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
            TestCase(4, vec![1, 4, 6, 3], 5),
            TestCase(2, vec![1000000000, 1], 999999999),
            TestCase(5, vec![1, 1, 1, 1, 1], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
