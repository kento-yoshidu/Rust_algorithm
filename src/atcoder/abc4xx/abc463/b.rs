// https://atcoder.jp/contests/abc463/tasks/abc463_b

fn run(_n: usize, x: char, s: Vec<&str>) -> &'static str {
    if s.into_iter()
        .any(|str| {
            let pos = (x as usize) - ('A' as usize);

            str.chars().nth(pos).unwrap() == 'o'
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, char, Vec<&'static str>, &'static str);

    #[test]
    fn abc463_b() {
        let tests = [
            TestCase(3, 'A', vec!["xoxox", "xxooo", "oxxxx"], "Yes"),
            TestCase(5, 'C', vec!["xoxoo", "oxxoo", "oxxxo", "xoxxx", "oxxoo"], "No"),
        ];

        for TestCase(n, x, s, expected) in tests {
            assert_eq!(run(n, x, s), expected);
        }
    }
}
