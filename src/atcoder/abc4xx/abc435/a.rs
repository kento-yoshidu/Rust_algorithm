// https://atcoder.jp/contests/abc435/tasks/abc435_a

fn run(n: usize) -> usize {
    (1..=n)
         .fold(0, |acc, i| acc + i)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc435_a() {
        let tests = [
            TestCase(5, 15),
            TestCase(1, 1),
            TestCase(29, 435),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
