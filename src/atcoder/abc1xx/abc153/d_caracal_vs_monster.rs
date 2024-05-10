// https://atcoder.jp/contests/abc153/tasks/abc153_d

fn calc(h: usize, count: usize) -> usize {
    if h < count {
        count - 1
    } else {
        calc(h, count*2)
    }
}

fn run(h: usize) -> usize {
    calc(h, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 7),
            TestCase(2, 3),
            TestCase(1000000000000, 1099511627775),
        ];

        for TestCase(h, expected) in tests {
            assert_eq!(run(h), expected);
        }
    }
}
