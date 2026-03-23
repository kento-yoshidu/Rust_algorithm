// https://atcoder.jp/contests/abc272/tasks/abc272_a

fn run(n: usize) -> String {
    format!("{:0>2X}", n)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc271_a() {
        let tests = [
            TestCase(99, "63"),
            TestCase(12, "0C"),
            TestCase(0, "00"),
            TestCase(255, "FF"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
