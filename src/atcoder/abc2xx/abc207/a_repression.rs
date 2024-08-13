// https://atcoder.jp/contests/abc207/tasks/abc207_a

fn run(a: usize, b: usize, c: usize) -> usize {
    let mut vec = vec![a, b, c];

    vec.sort();
    vec.reverse();

    vec[0] + vec[1]
}

fn run2(a: usize, b: usize, c: usize) -> usize {
    let sum = a + b + c;

    sum - a.min(b.min(c))
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, 5, 9),
            TestCase(6, 6, 6, 12),
            TestCase(99, 99, 98, 198),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
            assert_eq!(run2(a, b, c), expected);
        }
    }
}
