// https://atcoder.jp/contests/abc302/tasks/abc302_a

fn run(hp: usize, at: usize) -> usize {
    if hp % at == 0 {
        hp / at
    } else {
        hp / at + 1
    }
}

fn run2(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 3, 3),
            TestCase(123456789123456789, 987654321, 124999999),
            TestCase(999999999999999998, 2, 499999999999999999),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
            assert_eq!(run2(a, b), expected);
        }
    }
}
