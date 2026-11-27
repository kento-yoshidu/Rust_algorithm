// https://atcoder.jp/contests/awc0049/tasks/awc0049_a

fn run(_n: usize, l: usize, p: usize, q: usize, s: Vec<usize>) -> Vec<usize> {
    s.into_iter()
        .map(|s| {
            if s <= l {
                s * p / 100
            } else {
                (l * p + (s - l) * q) / 100
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn awc0049_a() {
        let tests = [
            TestCase(3, 100, 10, 23, vec![50, 100, 150], vec![5, 10, 21]),
            TestCase(4, 200, 30, 10, vec![100, 200, 300, 1], vec![30, 60, 70, 0]),
            TestCase(5, 5000, 15, 40, vec![0, 3000, 5000, 8000, 12345], vec![0, 450, 750, 1950, 3688]),
            TestCase(8, 1000000000, 5, 99, vec![0, 1, 999999999, 1000000000, 1000000000, 999999997, 500000000, 123456789], vec![0, 0, 49999999, 50000000, 50000000, 49999999, 25000000, 6172839]),
            TestCase(6, 1, 0, 0, vec![0, 1, 1000000000, 0, 0, 1], vec![0, 0, 0, 0, 0, 0]),
        ];

        for TestCase(n, l, p, q, s, expected) in tests {
            assert_eq!(run(n, l, p, q, s), expected);
        }
    }
}
