// https://atcoder.jp/contests/abc275/tasks/abc275_d

pub fn run(y: usize) -> &'static str {
    if y % 400 == 0 {
        return "YES"
    } else {
        if y % 4 == 0 && y % 100 != 0 {
            "YES"
        } else {
            "NO"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1001, "NO"),
            TestCase(2012, "YES"),
            TestCase(2100, "NO"),
            TestCase(2000, "YES"),
        ];

        for TestCase(y, expected) in tests {
            assert_eq!(run(y), expected);
        }
    }
}

