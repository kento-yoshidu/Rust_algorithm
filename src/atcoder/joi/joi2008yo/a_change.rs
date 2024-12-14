// https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_a

fn run(n: usize) -> usize {
    let vec = vec![500, 100, 50, 10, 5, 1];

    vec.into_iter()
        .fold((0, 1000-n), |acc, x| {
            (acc.0 + acc.1 / x, acc.1 % x)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(380, 4),
            TestCase(1, 15),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
