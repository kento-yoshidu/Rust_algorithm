// https://atcoder.jp/contests/joi2022yo1b/tasks/joi2022_yo1b_b

fn run(a: usize, b: usize) -> usize {
    if (a + b) % 12 == 0 {
        12
    } else {
        (a + b) % 12
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(9, 5, 2),
            TestCase(4, 20, 12),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
