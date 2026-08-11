// https://atcoder.jp/contests/abc333/tasks/abc333_a

fn run(n :usize) -> String {
    n.to_string().repeat(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc333_a() {
        let tests = [
            TestCase(1, "1"),
            TestCase(2, "22"),
            TestCase(3, "333"),
            TestCase(9, "999999999"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
