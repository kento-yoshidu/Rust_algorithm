// https://atcoder.jp/contests/joi2024yo1c/tasks/joi2024_yo1c_b

fn run(a: usize, b: usize) -> usize {
    (a + b).to_string().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 9, 2),
            TestCase(499, 499, 3),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
