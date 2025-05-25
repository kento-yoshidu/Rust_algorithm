// https://atcoder.jp/contests/abc076/tasks/abc076_b

fn run(n: usize, k: usize) -> usize {
    (1..=n).fold(1, |sum, _| {
        if sum >= k {
            sum + k
        } else {
            sum * 2
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 3, 10),
            TestCase(10, 10, 76),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
