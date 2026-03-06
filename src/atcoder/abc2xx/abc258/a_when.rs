// https://atcoder.jp/contests/abc258/tasks/abc258_a

fn run(n: usize) -> String {
    if n < 60 {
        format!("21:{:02}", n)
    } else {
        format!("22:{:02}", n - 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc258_a() {
        let tests = [
            TestCase(45, "21:45"),
            TestCase(60, "22:00"),
            TestCase(61, "22:01"),
            TestCase(90, "22:30"),
            TestCase(100, "22:40"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
