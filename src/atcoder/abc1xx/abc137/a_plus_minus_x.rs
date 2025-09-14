// https://atcoder.jp/contests/abc137/tasks/abc137_a

fn run(a: isize, b: isize) -> isize {
    [a+b, a-b, a*b].into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, isize, isize);

    #[test]
    fn abc137_a() {
        let tests = [
            TestCase(-13, 3, -10),
            TestCase(1, -33, 34),
            TestCase(13, 3, 39),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
