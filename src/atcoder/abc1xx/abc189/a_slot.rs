// https://atcoder.jp/contests/abc189/tasks/abc189_a

fn run(s: &str) -> &'static str {
    let mut c = s.chars();

    let l = c.next().unwrap();
    let m = c.next().unwrap();
    let r = c.next().unwrap();

    if l == m && m == r {
        "Won"
    } else {
        "Lost"
    }
}

fn run2(s: &str) -> &'static str {
    let mut chars: Vec<char> = s.chars().collect();

    chars.dedup();

    if chars.len() == 1 {
        "Won"
    } else {
        "Lost"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc189_a() {
        let tests = [
            TestCase("SSS", "Won"),
            TestCase("WVW", "Lost"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
