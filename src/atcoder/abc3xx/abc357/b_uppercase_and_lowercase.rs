// https://atcoder.jp/contests/abc357/tasks/abc357_b

fn run(s: &str) -> String {
    let count = s.chars()
        .fold((0, 0), |mut tup, c| {
            if c.is_uppercase() {
                tup.1 += 1;
                tup
            } else {
                tup.0 += 1;
                tup
            }
        });

    if count.0 < count.1 {
        s.to_uppercase()
    } else {
        s.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("AtCoder", "atcoder"),
            TestCase("SunTORY", "SUNTORY"),
            TestCase("a", "a"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
