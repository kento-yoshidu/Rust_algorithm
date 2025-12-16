// https://atcoder.jp/contests/abc248/tasks/abc248_b

fn calc(count: usize, a: usize, b: usize, k: usize) -> usize {
    if a >= b {
        count
    } else {
        calc(count+1, a*k, b, k)
    }
}

fn run(a: usize, b: usize, k: usize) -> usize {
    calc(0, a, b, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn abc248_b() {
        let tests = [
            TestCase(1, 4, 2, 2),
            TestCase(7, 7, 10, 0),
            TestCase(31, 415926, 5, 6),
            TestCase(158260522, 200224287, 10, 1),
            TestCase(1, 1000000000, 2, 30),
            TestCase(999999999, 1000000000, 500000000, 1),
            TestCase(1, 536870912, 2, 29),
        ];

        for TestCase(a, b, k, expected) in tests {
            assert_eq!(run(a, b, k), expected);
        }
    }
}
