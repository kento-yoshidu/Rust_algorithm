// https://atcoder.jp/contests/abc120/tasks/abc120_a

fn run(a: usize, b: usize, c: usize) -> usize {
    if b / a >= c {
        c
    } else {
        b / a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn abc120_a() {
        let tests = [
            TestCase(2, 11, 4, 4),
            TestCase(3, 9, 5, 3),
            TestCase(100, 1, 10, 0),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
