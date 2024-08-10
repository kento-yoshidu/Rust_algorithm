// https://atcoder.jp/contests/abc235/tasks/abc235_a

fn run(n: usize) -> u32 {
    n.to_string()
        .chars()
        .map(|c| {
            let num = c.to_digit(10).unwrap();

            num * 100 + num * 10 + num
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, u32);

    #[test]
    fn test() {
        let tests = [
            TestCase(123, 666),
            TestCase(999, 2997),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
