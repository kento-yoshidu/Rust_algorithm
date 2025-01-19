// https://atcoder.jp/contests/abc006/tasks/abc006_2

fn run(n: usize) -> usize {
    (0..n-1)
        .fold((0, 0, 1), |state, _| {
            let sum = state.0 + state.1 + state.2;

            (state.1 % 10007, state.2 % 10007, sum % 10007)
        }).0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 7),
            TestCase(2, 0),
            TestCase(100000, 7927),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n),  expected);
        }
    }
}
