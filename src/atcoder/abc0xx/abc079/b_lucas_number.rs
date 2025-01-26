// https://atcoder.jp/contests/abc079/tasks/abc079_b

fn run(n: usize) -> usize {
    (1..n)
        .fold((2, 1), |(l, r), _| {
            (r, l+r)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 11),
            TestCase(86, 939587134549734843),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
