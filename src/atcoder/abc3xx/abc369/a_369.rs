// https://atcoder.jp/contests/abc369/tasks/abc369_a

fn run(a: isize, b: isize) -> usize {
    if a == b {
        1
    } else if (b - a).abs() % 2 == 0 {
        3
    } else {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 7, 3),
            TestCase(6, 1, 2),
            TestCase(3, 3, 1),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
