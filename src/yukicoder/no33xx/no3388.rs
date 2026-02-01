// https://yukicoder.me/problems/no/3388

fn is_prime(n: usize) -> bool {
    n >= 2 && (2..n).all(|i| n % i != 0)
}

fn run(a: usize, b: usize) -> usize {
    (a..=b)
        .filter(|x| is_prime(*x))
        .map(|x| x*x*x - x*x + x + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn yuki_3388() {
        let tests = [
            TestCase(2, 8, 437),
            TestCase(8, 10, 0),
            TestCase(29, 29, 23578),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
