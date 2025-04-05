// https://atcoder.jp/contests/abc043/tasks/abc043_b

fn run(s: &str) -> String {
    let mut ans = Vec::new();

    for c in s.chars() {
        if c == '0' {
            ans.push('0');
        } else if c == '1' {
            ans.push('1');
        } else {
            ans.pop();
        }
    }

    ans.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::atcoder::arc0xx::arc004::a_the_longest_distance;

    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("01B0", "00"),
            TestCase("0BB1", "1"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
