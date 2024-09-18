// https://atcoder.jp/contests/abc300/tasks/abc300_a

fn run(_n: usize, a: usize, b: usize, vec: Vec<usize>) -> usize {
    vec
        .into_iter()
        .position(|x| x == a+b)
        .unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 125, 175, vec![200, 300, 400], 2),
            TestCase(1, 1, 1, vec![2], 1),
            TestCase(5, 123, 456, vec![135, 246, 357, 468, 579], 5),
        ];

        for TestCase(n, a, b, vec, expected) in tests {
            assert_eq!(run(n, a, b, vec), expected);
        }
    }
}
