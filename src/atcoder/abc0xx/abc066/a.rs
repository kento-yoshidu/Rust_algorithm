// https://atcoder.jp/contests/abc066/tasks/abc066_a

fn run(a: usize, b: usize, c: usize) -> usize {
    let sum = a + b + c;

    let max = vec![a, b, c].into_iter().max().unwrap();

    sum - max
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn abc066_a() {
        let tests = [
            TestCase(700, 600, 780, 1300),
            TestCase(10000, 10000, 10000, 20000),
        ];

        for TestCase(a, b, c, expected) in tests {
            assert_eq!(run(a, b, c), expected);
        }
    }
}
