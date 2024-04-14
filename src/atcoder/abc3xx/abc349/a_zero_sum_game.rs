// https://atcoder.jp/contests/abc349/tasks/abc349_a

fn run(_n: usize, a: Vec<isize>) -> isize {
    let sum: isize = a.iter().sum();

    0 - sum
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![1, -2, -1], 2),
            TestCase(3, vec![0, 0], 0),
            TestCase(6, vec![10, 20, 30, 40, 50], -150),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
