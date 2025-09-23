// https://atcoder.jp/contests/abc146/tasks/abc146_b

fn run(n: u8, str: &str) -> String {
    let mut result = String::from("");

    for c in str.chars() {
        let mut tmp = c as u8 + n;

        if tmp as u8 > 90 {
            tmp -= 26;
        }

        result.push(tmp as char);
    }

    result
}

fn run2(n: u8, str: &str) -> String {
    str
        .chars()
        .map(|c| {
            c as u8 + n
        })
        .map(|c| {
            if c > 90 {
                (c - 26) as char
            } else {
                c as char
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u8, &'static str, &'static str);

    #[test]
    fn abc146_b() {
        let tests = [
            TestCase(2, "ABCXYZ", "CDEZAB"),
            TestCase(0, "ABCXYZ", "ABCXYZ"),
            TestCase(13, "ABCDEFGHIJKLMNOPQRSTUVWXYZ", "NOPQRSTUVWXYZABCDEFGHIJKLM"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
        }
    }
}
