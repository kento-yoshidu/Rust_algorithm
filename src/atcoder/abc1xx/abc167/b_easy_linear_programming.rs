// https://atcoder.jp/contests/abc167/tasks/abc167_b

fn run(a: isize, b: isize, _c: isize, k: isize) -> isize {
    if a >= k {
        k
    } else if a + b >= k {
        a
    } else {
        a - (k - (a + b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize);

    #[test]
    fn abc167_b() {
        let tests = [
            TestCase(2, 1, 1, 3, 2),
            TestCase(1, 2, 3, 4, 0),
            TestCase(2000000000, 0, 0, 2000000000, 2000000000),
            TestCase(0, 0, 2000000000, 2000000000, -2000000000),
        ];

        for TestCase(a, b, c, k, expected) in tests {
            assert_eq!(run(a, b, c, k), expected);
        }
    }
}
