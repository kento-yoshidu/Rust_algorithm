// https://atcoder.jp/contests/joi2020yo1a/tasks/joi2020_yo1a_a

fn run(a: usize, b: usize, c: usize) -> usize {
    if a == b || a == c {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 2, 1, 1),
            TestCase(2, 2, 2, 2),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
