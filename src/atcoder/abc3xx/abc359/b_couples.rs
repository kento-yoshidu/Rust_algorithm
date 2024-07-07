// https://atcoder.jp/contests/abc359/tasks/abc359_b

pub fn run(n: usize, a: Vec<usize>) -> usize {
    (0..n*2-2)
        .filter(|i| {
            a[*i] == a[i+2]
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
            TestCase(3, vec![1, 2, 1, 3, 2, 3], 2),
            TestCase(2, vec![1, 1, 2, 2], 0),
            TestCase(4, vec![4, 3, 2, 3, 2, 1, 4, 1], 3),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
