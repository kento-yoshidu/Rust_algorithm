// https://atcoder.jp/contests/abc170/tasks/abc170_c

fn run(x: i32, _n: i32, p: Vec<i32>) -> i32 {
    (0..=101)
        .filter(|i| !p.contains(i))
        .min_by_key(|&i| ((x-i).abs(), i))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(i32, i32, Vec<i32>, i32);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 5, vec![4, 7, 10, 6, 5], 8),
            TestCase(10, 5, vec![4, 7, 10, 6, 5], 9),
            TestCase(6, 4, vec![4, 7, 10, 5], 6),
            TestCase(100, 2, vec![100, 99], 101),
        ];

        for TestCase(x, n, p, expected) in tests {
            assert_eq!(run(x, n, p), expected);
        }
    }
}
