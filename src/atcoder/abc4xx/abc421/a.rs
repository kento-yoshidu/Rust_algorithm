// https://atcoder.jp/contests/abc421/tasks/abc421_a

fn run(_n: usize, s: Vec<&str>, x: usize, y: &str) -> &'static str {
    if s[x-1] == y {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize, &'static str, &'static str);

    #[test]
    fn abc421_a() {
        let tests = [
            TestCase(3, vec!["sato", "suzuki", "takahashi"], 3, "takahashi", "Yes"),
            TestCase(3, vec!["sato", "suzuki", "takahashi"], 1, "aoki", "No"),
            TestCase(2, vec!["smith", "smith"], 1, "smith", "Yes"),
            TestCase(2, vec!["wang", "li"], 2, "wang", "No"),
        ];

        for TestCase(n, s, x, y, expected) in tests {
            assert_eq!(run(n, s, x, y), expected);
        }
    }
}
