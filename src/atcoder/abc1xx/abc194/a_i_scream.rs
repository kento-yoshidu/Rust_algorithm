// https://atcoder.jp/contests/abc194/tasks/abc194_a

pub fn run(a: usize, b: usize) -> usize {
    if a + b >= 15 && b >= 8 {
        1
    } else if a + b >= 10 && b >= 3 {
        2
    } else if a + b >= 3 {
        3
    } else {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 8, 1),
            TestCase(1, 2, 3),
            TestCase(0, 0, 4),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
