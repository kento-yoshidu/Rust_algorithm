// https://atcoder.jp/contests/abc361/tasks/abc361_a

fn run(_n: usize, k: usize, x: usize, a: Vec<usize>) -> Vec<usize> {
    let mut vec = a.clone();

    vec.insert(k, x);

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 3, 7, vec![2, 3, 5, 11], vec![2, 3, 5, 7, 11]),
            TestCase(1, 1, 100, vec![100], vec![100, 100]),
            TestCase(8, 8, 3, vec![9, 9, 8, 2, 4, 4, 3, 5], vec![9, 9, 8, 2, 4, 4, 3, 5, 3]),
        ];

        for TestCase(n, k, x, a, expected) in tests {
            assert_eq!(run(n, k, x, a), expected);
        }
    }
}
