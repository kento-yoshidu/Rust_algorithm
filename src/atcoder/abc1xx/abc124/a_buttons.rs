// https://atcoder.jp/contests/abc124/tasks/abc124_a

fn run(a: usize, b: usize) -> usize {
    if a == b {
        return a * 2;
    }

    let large = a.max(b);

    large + large - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc124_a() {
        let tests = [
            TestCase(5, 3, 9),
            TestCase(3, 4, 7),
            TestCase(6, 6, 12),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
