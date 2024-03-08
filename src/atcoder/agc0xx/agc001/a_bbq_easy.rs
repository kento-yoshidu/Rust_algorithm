// https://atcoder.jp/contests/agc001/tasks/agc001_a

fn run(n: usize, l: Vec<usize>) -> usize {
    let mut vec = l.clone();
    vec.sort();

    (0..2*n).step_by(2)
        .map(|i| {
            vec[i]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, vec![1, 3, 1, 2], 3),
            TestCase(5, vec![100, 1, 2, 3, 14, 15, 58, 58, 58, 29], 135),
        ];

        for TestCase(n, l, expected) in tests {
            assert_eq!(run(n, l), expected);
        }
    }
}
