// https://atcoder.jp/contests/abc143/tasks/abc143_a

fn run(a: usize, b: usize) -> usize {
    if a <= b * 2 {
        0
    } else {
        a - b * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc143_a() {
        let tests = [
            TestCase(12, 4, 4),
            TestCase(20, 15, 0),
            TestCase(20, 30, 0),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
