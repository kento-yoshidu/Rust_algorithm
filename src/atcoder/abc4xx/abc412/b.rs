// https://atcoder.jp/contests/abc412/tasks/abc412_b

fn run(s: &str, t: &str) -> &'static str {
    let vec: Vec<(usize, char)> = s.chars()
        .enumerate()
        .filter(|(_, c)| c.is_uppercase())
        .collect();

    if vec.len() < 2 {
        return "Yes";
    }

    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    if vec.into_iter()
        .skip(1)
        .any(|(i, _)| {
            let char = s[i-1];

            !t.contains(&char)
        }) {
            "No"
        } else {
            "Yes"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn abc412_b() {
        let tests = [
            TestCase("AtCoder", "Total", "Yes"),
            TestCase("aBCdE", "abcdcba", "No"),
            TestCase("abcde", "XYZ", "Yes"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
