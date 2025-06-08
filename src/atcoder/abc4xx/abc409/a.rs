// https://atcoder.jp/contests/abc409/tasks/abc409_a

fn run(_n: usize, t: &str, a: &str) -> &'static str {
    if t.chars()
        .zip(a.chars())
        .any(|tpl| {
            tpl.0 == 'o' && tpl.1 == 'o'
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str, &'static str);

    #[test]
    fn abc409_a() {
        let tests = [
            TestCase(4, "oxoo", "xoox", "Yes"),
            TestCase(5, "xxxxx", "ooooo", "No"),
            TestCase(10, "xoooxoxxxo", "ooxooooxoo", "Yes"),
        ];

        for TestCase(n, t, a, expected) in tests {
            assert_eq!(run(n, t, a), expected);
        }
    }
}
