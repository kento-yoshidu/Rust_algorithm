// https://atcoder.jp/contests/abc168/tasks/abc168_b

fn run(k: usize, s: &str) -> String {
    if k >= s.len() {
        s.to_string()
    } else {
        format!("{}...", &s[0..k])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn abc168_b() {
        let tests = [
            TestCase(7, "nikoandsolstice", "nikoand..."),
            TestCase(40, "ferelibenterhominesidquodvoluntcredunt", "ferelibenterhominesidquodvoluntcredunt"),
        ];

        for TestCase(k, s, expected) in tests {
            assert_eq!(run(k, s), expected);
        }
    }
}
