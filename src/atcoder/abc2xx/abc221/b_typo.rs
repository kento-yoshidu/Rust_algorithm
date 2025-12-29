// https://atcoder.jp/contests/abc221/tasks/abc221_b

fn run(s: &str, t: &str) -> &'static str {
    if s == t {
        return "Yes";
    }

    for i in 0..t.len()-1 {
        let mut chars: Vec<char> = t.chars().collect();

        chars.swap(i, i+1);

        if s.chars().collect::<Vec<char>>() == chars {
            return "Yes";
        }
    }

    "No"
}

fn run2(s: &str, t: &str) -> &'static str {
    if s == t {
        return "Yes";
    }

    if (0..t.len()-1)
        .any(|i| {
            let mut chars: Vec<char> = t.chars().collect();

            chars.swap(i, i+1);

            s.chars().collect::<Vec<char>>() == chars
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
    fn abc221_b() {
        let tests = [
            TestCase("abc", "acb", "Yes"),
            TestCase("aabb", "bbaa", "No"),
            TestCase("abcde", "abcde", "Yes"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
            assert_eq!(run2(s, t), expected);
        }
    }
}
