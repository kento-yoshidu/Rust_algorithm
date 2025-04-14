// https://atcoder.jp/contests/joi2022yo1c/tasks/joi2022_yo1c_b

fn run(s: usize, a: usize, b: usize) -> usize {
    if s <= a {
        250
    } else {
        (s - a + b - 1) / b * 100 + 250
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(28, 20, 5, 450),
            TestCase(1, 100, 1, 250),
            TestCase(100, 1, 1, 10150),
        ];

        for TestCase(s, a, b, expected) in tests {
            assert_eq!(run(s, a, b), expected);
        }
    }
}
