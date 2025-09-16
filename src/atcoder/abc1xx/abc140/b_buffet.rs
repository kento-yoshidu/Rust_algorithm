// https://atcoder.jp/contests/abc140/tasks/abc140_b

fn run(n: usize, a: Vec<usize>, b: Vec<usize>, c: Vec<usize>) -> usize {
    (0..n)
        .fold((0, 999),  |(state, prev), i| {
            if prev + 1 == a[i] {
                (state + c[prev-1] + b[a[i]-1], a[i])
            } else {
                (state + b[a[i]-1], a[i])
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn abc140_b() {
        let tests = [
            TestCase(3, vec![3, 1, 2], vec![2, 5, 4], vec![3, 6], 14),
            TestCase(4, vec![2, 3, 4, 1], vec![13, 5, 8, 24], vec![45, 9, 15], 74),
            TestCase(2, vec![1, 2], vec![50, 50], vec![50], 150),
        ];

        for TestCase(n, a, b, c, expected) in tests {
            assert_eq!(run(n, a, b, c), expected);
        }
    }
}
