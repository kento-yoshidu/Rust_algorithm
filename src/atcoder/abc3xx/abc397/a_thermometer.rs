// https://atcoder.jp/contests/abc397/tasks/abc397_a

fn run(x: f64) -> usize {
    if x >= 40.0 {
        1
    } else if x >= 37.5 {
        2
    } else {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(f64, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(40.0, 1),
            TestCase(37.7, 2),
            TestCase(36.6, 3),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
