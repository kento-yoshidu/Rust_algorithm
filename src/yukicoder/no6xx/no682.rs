// https://yukicoder.me/problems/no/682

fn run(a: usize, b: usize) -> usize {
    (a..=b)
        .filter(|n| (a + b + n) % 3 == 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn yuki_682() {
        let tests = [
            TestCase(2, 8, 3),
            TestCase(2, 3, 0),
            TestCase(1, 100, 34),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
