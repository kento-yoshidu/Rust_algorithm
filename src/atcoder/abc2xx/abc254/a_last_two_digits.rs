// https://atcoder.jp/contests/abc254/tasks/abc254_a

fn run(n: usize) -> String {
    format!("{:0>2}", n % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCaes(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCaes(254, "54"),
            TestCaes(101, "01"),
        ];

        for TestCaes(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
