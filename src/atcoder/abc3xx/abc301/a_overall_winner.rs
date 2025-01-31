// https://atcoder.jp/contests/abc300/tasks/abc300_a

fn run(n: usize, s: &str) -> &'static str {
    let mut t = 0;
    let mut a = 0;

    for c in s.chars() {
        if c == 'A' {
            a += 1;
        } else {
            t += 1;
        }

        if t == (n + 1) / 2 {
            return "T";
        }

        if a == (n + 1) / 2 {
            return "A";
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, "TTAAT", "T"),
            TestCase(6, "ATTATA", "T"),
            TestCase(1, "A", "A"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
