// https://atcoder.jp/contests/nomura2020/tasks/nomura2020_b

pub fn run(t: &str) -> String {
    let mut ans = String::new();

    for c in t.chars() {
        match c {
            '?' => {
                ans.push('D');
            },
            _ => {
                ans.push(c);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("PD?D??P", "PDDDDDP"),
            TestCase("P?P?", "PDPD"),
        ];

        for TestCase(t, expected) in tests {
            assert_eq!(run(t), expected);
        }
    }
}
