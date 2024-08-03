// https://atcoder.jp/contests/abc365/tasks/abc365_a

fn run(y: usize) -> usize {
    if y % 400 == 0 || (y % 4 == 0 && y % 100 != 0) {
        366
    } else {
        365
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2023, 365),
            TestCase(1992, 366),
            TestCase(1800, 365),
            TestCase(1600, 366),
        ];

        for TestCase(y, expected) in tests {
            assert_eq!(run(y), expected);
        }
    }
}
