// https://atcoder.jp/contests/abc445/tasks/abc445_b

fn run(_n: usize, s: Vec<&str>) -> Vec<String> {
    let len = s.iter().map(|s| s.len()).max().unwrap();

    s.into_iter()
        .map(|s| {
            let p = len - s.len();

            format!("{}{}{}", ".".repeat(p/2), s, ".".repeat(p/2))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn abc445_b() {
        let tests = [
            TestCase(4, vec!["apple", "blueberry", "coconut", "dragonfruit"], vec![ "...apple...", ".blueberry.", "..coconut..", "dragonfruit"]),
            TestCase(6, vec!["abc", "d", "efghi", "jkl", "mnopq", "r"], vec![".abc.", "..d..", "efghi", ".jkl.", "mnopq", "..r.."]),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
