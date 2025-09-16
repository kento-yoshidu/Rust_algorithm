// https://atcoder.jp/contests/abc139/tasks/abc139_b

fn run(a: usize, b: usize) -> usize {
    if (b - 1) % (a - 1) == 0 {
        (b - 1) / (a - 1)
    } else {
        (b - 1) / (a - 1) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc138_b() {
        let tests = [
            TestCase(4, 10, 3),
            TestCase(8, 9, 2),
            TestCase(8, 8, 1),
            TestCase(4, 12, 4),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
