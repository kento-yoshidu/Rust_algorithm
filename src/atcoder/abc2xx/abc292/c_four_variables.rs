// https://atcoder.jp/contests/abc292/tasks/abc292_c

fn run(n: usize) -> usize {
    let mut ab = vec![0; n+1];

    for i in 1..=n {
        for j in 1..=(n / i) {
            ab[i*j] += 1;
        }
    }

    (1..=n)
        .map(|i| {
            ab[i] * ab[n-i]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 8),
            TestCase(292, 10886),
            TestCase(19876, 2219958),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
