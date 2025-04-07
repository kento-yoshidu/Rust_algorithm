// https://atcoder.jp/contests/abc049/tasks/arc065_a

fn run(s: &str) -> &'static str {
    let strings = ["dream", "dreamer", "erase", "eraser"];

    let mut dp = vec![false; s.len()+1];
    dp[0] = true;

    for i in 0..s.len() {
        if dp[i] == false {
            continue;
        }

        for str in strings {
            if str.len() + i > s.len() {
                continue;
            }

            if str == &s[i..i+str.len()] {
                dp[i+str.len()] = true;
            }

            if dp[s.len()] == true {
                return "YES"
            }
        }
    }

    "NO"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("erasedream", "YES"),
            TestCase("dreameraser", "YES"),
            TestCase("dreamerer", "NO"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
