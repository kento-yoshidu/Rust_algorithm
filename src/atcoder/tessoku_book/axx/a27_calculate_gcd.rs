// https://atcoder.jp/contests/tessoku-book/tasks/math_and_algorithm_o

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn run(a: usize, b: usize) -> usize {
    gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn tessoku_a27() {
        let tests = [
            TestCase(900, 700, 100),
            TestCase(117, 432, 9),
            TestCase(998244353, 1000000000, 1),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
