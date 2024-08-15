// https://atcoder.jp/contests/abc110/tasks/abc110_a

fn run(a: usize, b: usize, c: usize) -> usize {
    let mut vec = vec![a, b, c];

    vec.sort();

    vec[2] * 10 + vec[0] + vec[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 5, 2, 53),
            TestCase(9, 9, 9, 108),
            TestCase(6, 6, 7, 82),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
