// https://atcoder.jp/contests/abc015/tasks/abc015_2

fn run(_n: usize, a: Vec<usize>) -> usize {
    let sum: usize = a.iter().sum();
    let count = a.iter().filter(|n| **n != 0).count();

    (sum as f64 / count as f64).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![0, 1, 3, 8], 4),
            TestCase(5, vec![1, 4, 9, 10, 15], 8),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
