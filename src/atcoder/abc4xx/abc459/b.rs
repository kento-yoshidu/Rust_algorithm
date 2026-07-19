// https://atcoder.jp/contests/abc459/tasks/abc459_b

fn run(_n: usize, s: Vec<&str>) -> String {
    s.into_iter()
        .map(|str| {
            match str.chars().nth(0).unwrap() {
                ..='c' => '2',
                ..='f' => '3',
                ..='i' => '4',
                ..='l' => '5',
                ..='o' => '6',
                ..='s' => '7',
                ..='v' => '8',
                ..='z' => '9',
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, &'static str);

    #[test]
    fn abc459_b() {
        let tests = [
            TestCase(2, vec!["algorithm", "heuristic"], "24"),
            TestCase(3, vec!["i", "love", "you"], "459"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
