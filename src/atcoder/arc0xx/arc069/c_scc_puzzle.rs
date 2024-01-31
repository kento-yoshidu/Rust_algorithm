// https://atcoder.jp/contests/abc055/tasks/arc069_a

pub fn run(n: isize, m: isize) -> isize {
    if n >= m {
        m / 2
    } else {
        n + (m - n*2) / 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 6, 2),
            TestCase(10, 10, 5),
            TestCase(12345, 678901, 175897),
            TestCase(1000000000000, 1000000000000, 500000000000),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(expected, run(n, m));
        }
    }
}
