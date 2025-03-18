// https://atcoder.jp/contests/abc053/tasks/arc068_a

fn run(x: usize) -> usize {
    let count = (x / 11) * 2;

    if x % 11 == 0 {
        count
    } else if x % 11 > 6 {
        count + 2
    } else {
        count + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 2),
            TestCase(22, 4),
            TestCase(149696127901, 27217477801),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
