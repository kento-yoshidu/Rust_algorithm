// https://atcoder.jp/contests/abc222/tasks/abc222_a

fn run(n: usize) -> String {
    format!("{:04}", n)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc222_a() {
        let tests = [
            TestCase(321, "0321"),
            TestCase(7777, "7777"),
            TestCase(1, "0001"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
