// https://atcoder.jp/contests/abc196/tasks/abc196_a

fn run(a: isize, b: isize, c: isize, d: isize) -> isize {
    let max = a.max(b);
    let min = c.min(d);

    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize, isize, isize);

    #[test]
    fn abc196_a() {
        let tests = [
            TestCase(0, 10, 0, 10, 10),
            TestCase(-100, -100, 100, 100, -200),
            TestCase(-100, 100, -100, 100, 200),
        ];

        for TestCase(a, b, c, d, expected) in tests {
            assert_eq!(run(a, b, c, d), expected);
        }
    }
}
