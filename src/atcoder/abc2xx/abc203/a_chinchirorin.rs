// https://atcoder.jp/contests/abc203/tasks/abc203_a

fn run(a: usize, b: usize, c: usize) -> usize {
    if a != b && b != c && a != c {
        return 0;
    }

    if a == b {
        c
    } else if b == c {
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
    fn abc203_a() {
        let tests = [
            TestCase(2, 5, 2, 5),
            TestCase(4, 5, 6, 0),
            TestCase(1, 1, 1, 1),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
