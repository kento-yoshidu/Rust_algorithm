// https://atcoder.jp/contests/abc436/tasks/abc436_a

fn run(n: usize, s: &str) -> String {
    format!("{:o>width$}", s, width = n)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn abc436_a() {
        let tests = [
            TestCase(5, "abc", "ooabc"),
            TestCase(2, "o", "oo"),
            TestCase(12, "vgxgpuam", "oooovgxgpuam"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
