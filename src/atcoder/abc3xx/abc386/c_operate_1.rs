// https://atcoder.jp/contests/abc386/tasks/abc386_c

fn check(short: &Vec<char>, long: &Vec<char>) -> &'static str {
    let mut pos_s = 0;
    let mut pos_l = 0;
    let mut mismatch_count = 0;

    while pos_s < short.len() && pos_l < long.len() {
        if short[pos_s] != long[pos_l] {
            mismatch_count += 1;
            if mismatch_count > 1 {
                return "No";
            }
            pos_l += 1;
        } else {
            pos_s += 1;
            pos_l += 1;
        }
    }
    "Yes"
}

fn run(_k: usize, s: &str, t: &str) -> &'static str {
    if s == t {
        return "Yes";
    }

    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    match s.len().cmp(&t.len()) {
        std::cmp::Ordering::Equal => {
            let mismatch_count = s.iter()
                .zip(&t)
                .filter(|(a, b)| a != b)
                .count();

            if mismatch_count == 1 {
                "Yes"
            } else {
                "No"
            }
        }
        std::cmp::Ordering::Less if s.len() + 1 == t.len() => check(&s, &t),
        std::cmp::Ordering::Greater if s.len() == t.len() + 1 => check(&t, &s),
        _ => "No",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, "abc", "agc", "Yes"),
            TestCase(1, "abc", "awtf", "No"),
            TestCase(1, "abc", "ac", "Yes"),
            TestCase(1, "back", "black", "Yes"),
            TestCase(1, "leap", "read", "No"),
            TestCase(1, "aabb", "aaa", "No"),
        ];

        for TestCase(k, s, t, expected) in tests {
            assert_eq!(run(k, s, t), expected);
        }
    }
}
