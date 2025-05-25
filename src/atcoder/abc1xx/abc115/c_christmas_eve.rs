// https://atcoder.jp/contests/abc115/tasks/abc115_c

fn run(n: usize, k: usize, vec: Vec<usize>) -> usize {
    let mut vec = vec.clone();

    vec.sort();

    (0..=n - k).map(|i| {
        vec[i + k-1] - vec[i]
    })
    .min()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 3, &mut vec![10, 15, 11, 14, 12], 2),
            TestCase(5, 3, &mut vec![5, 7, 5, 7, 7], 0),
        ];

        for TestCase(n, k, vec, expected) in tests {
            assert_eq!(run(n, k, vec), expected);
        }
    }
}
