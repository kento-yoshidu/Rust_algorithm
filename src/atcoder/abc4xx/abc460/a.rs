// https://atcoder.jp/contests/abc460/tasks/abc460_a

fn calc(n: usize, m: usize, count: usize) -> usize {
    if m == 0 {
        count
    } else {
        calc(n, n % m, count+1)
    }
}

fn run(n: usize, m: usize) -> usize {
    calc(n, m, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc460_a() {
        let tests = [
            TestCase(8, 5, 3),
            TestCase(14, 6, 2),
            TestCase(460, 33, 5),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
