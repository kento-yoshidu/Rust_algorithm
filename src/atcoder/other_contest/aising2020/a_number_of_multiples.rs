// https://atcoder.jp/contests/aising2020/tasks/aising2020_a

fn run(l: usize, r: usize, d: usize) -> usize {
    (l..=r).filter(|num| {
        num % d == 0
    })
    .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 10, 2, 3),
            TestCase(6, 20, 7, 2),
            TestCase(1, 100, 1, 100),
        ];

        for TestCase(l, r, d, expected) in tests {
            assert_eq!(run(l, r, d), expected);
        }
    }
}
