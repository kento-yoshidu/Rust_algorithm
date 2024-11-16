// https://atcoder.jp/contests/abc027/tasks/abc027_a

fn run(a: usize, b: usize, c: usize) -> usize {
    a ^ b ^ c
}

fn run2(a: usize, b: usize, c: usize) -> usize {
    if a == b && b == c {
        return a
    }

    if a == b && b != c {
        c
    } else if a != b && b == c {
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
            TestCase(1, 1, 2, 2),
            TestCase(4, 3, 4, 3),
            TestCase(5, 5, 5, 5),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
            assert_eq!(run2(a, b, c), expected);
        }
    }
}
