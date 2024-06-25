// https://atcoder.jp/contests/abc355/tasks/abc355_a

pub fn run(a: isize, b: isize) -> isize {
    if a == b {
        -1
    } else {
        a ^ b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 2, 3),
            TestCase(1, 1, -1),
            TestCase(3, 1, 2),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}