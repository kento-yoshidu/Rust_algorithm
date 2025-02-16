// https://atcoder.jp/contests/abc393/tasks/abc393_b

fn run(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut ans = 0;

    for i in 0..s.len() {
        for j in i+1..s.len() {
            for k in j+1..s.len() {
                if j - i == k - j {
                    if chars[i] == 'A' && chars[j] == 'B' && chars[k] == 'C' {
                        ans += 1;
                    }
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("AABCC", 2),
            TestCase("ARC", 0),
            TestCase("AABAAABBAEDCCCD", 4),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
