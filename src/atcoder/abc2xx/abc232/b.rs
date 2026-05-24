// https://atcoder.jp/contests/abc232/tasks/abc232_b

fn next_char(c: char, n: u8) -> char {
    let val = (c as u8 - 97 + n) % 26 + 97;
    val as char
}

fn run(s: &str, t: &str) -> &'static str {
    if (0..26)
        .any(|i| {
            let str: String = s.chars().map(|c| next_char(c, i)).collect();

            str == t
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn abc232_b() {
        let tests = [
            TestCase("abc", "ijk", "Yes"),
            TestCase("z", "a", "Yes"),
            TestCase("ppq", "qqp", "No"),
            TestCase("atcoder", "atcoder", "Yes"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
