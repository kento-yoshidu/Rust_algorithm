// https://atcoder.jp/contests/abc012/tasks/abc012_2

fn run(n: usize) -> String {
    let h = n / (60 * 60);
    let m = n / 60 % 60;
    let s = n % 60;

    format!("{:02}:{:02}:{:02}", h, m, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3661, "01:01:01"),
            TestCase(86399, "23:59:59"),
            TestCase(86398, "23:59:58"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
