// https://atcoder.jp/contests/abc259/tasks/abc259_c

fn run_lengths(s: Vec<char>) -> Vec<(char, usize)> {
    let mut i = 0;
    let mut run_lengths = vec![];
    let mut current = (s[0], 0);

    loop {
        while i < s.len() && s[i] == current.0 {
            current.1 += 1;
            i += 1;
        }

        run_lengths.push(current);

        if i == s.len() {
            break;
        } else {
            current = (s[i], 0);
        }
    }

    run_lengths
}

fn run(s: &str, t: &str) -> &'static str {
    let s_length = run_lengths(s.chars().collect());
    let t_length = run_lengths(t.chars().collect());

    if s_length.len() != t_length.len() {
        return "No";
    }

    if s_length.into_iter()
        .zip(t_length.into_iter())
        .any(|(s, t)| {
            // 文字が違う場合
            // tが長さ2以上なのにsが長さ1しかない場合
            // sの方が長い場合（tを増やすことはできないので）
            s.0 != t.0 || (t.1 > 1 && s.1 == 1) || s.1 > t.1
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
    fn test() {
        let tests = [
            TestCase("abbaac", "abbbbaaac", "Yes"),
            TestCase("xyzz", "xyyzz", "No"),
            TestCase("aa", "aa", "Yes"),
            TestCase("aa", "aabb", "No"),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
