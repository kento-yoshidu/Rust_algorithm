// https://atcoder.jp/contests/abc230/tasks/abc230_b

fn run(s: &str) -> &'static str {
    let len = s.len();

    let str = "oxxoxxoxxoxx";

    for i in 0..3 {
        if &str[i..i+len] == s {
            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc230_b() {
        let tests = [
            TestCase("xoxxoxxo", "Yes"),
            TestCase("xxoxxoxo", "No"),
            TestCase("ox", "Yes"),
            TestCase("oo", "No"),
            TestCase("oxxoxxoxxo", "Yes"),
            TestCase("oxxoxxoxoo", "No"),
            TestCase("xxooxox", "No"),
            TestCase("ooxoxxox", "No"),
            TestCase("ooxxoxx", "No"),
            TestCase("x", "Yes"),
            TestCase("o", "Yes"),
            TestCase("xo", "Yes"),
            TestCase("xx", "Yes"),
            TestCase("xxx", "No"),
            TestCase("xxoxxoxo", "No"),
            TestCase("oox", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
