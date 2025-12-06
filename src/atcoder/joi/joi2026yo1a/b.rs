// https://atcoder.jp/contests/joi2026yo1a/tasks/joi2026_yo1a_b

fn run(a: usize, b: usize) -> usize {
    if a > b * 3 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn joi2026yo1b() {
        let tests = [
            TestCase(100, 20, 1),
            TestCase(70, 30, 0),
            TestCase(30, 10, 0),
            TestCase(76, 25, 1)
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
