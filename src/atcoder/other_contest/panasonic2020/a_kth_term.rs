// https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_a

fn run(k: usize) -> usize {
    let vec = vec![1, 1, 1, 2, 1, 2, 1, 5, 2, 2, 1, 5, 1, 2, 1, 14, 1, 5, 1, 5, 2, 2, 1, 15, 2, 2, 5, 4, 1, 4, 1, 51];

    vec[k-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, 2),
            TestCase(27, 5),
        ];

        for TestCase(k, expected) in tests {
            assert_eq!(run(k), expected);
        }
    }
}
