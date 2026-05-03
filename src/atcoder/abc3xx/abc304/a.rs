// https://atcoder.jp/contests/abc304/tasks/abc304_a

fn run<'a>(n: usize, sa: Vec<(&'a str, usize)>) -> Vec<&'a str> {
    let min = sa.iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.1.cmp(&b.1))
        .map(|(i, _)| i)
        .unwrap();

    (0..n)
        .map(|i| {
            sa[(i + min) % n].0
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(&'static str, usize)>, Vec<&'static str>);

    #[test]
    fn abc304_a() {
        let tests = [
            TestCase(5, vec![("alice", 31), ("bob", 41), ("carol", 5), ("dave", 92), ("ellen", 65)],vec!["carol", "dave", "ellen", "alice", "bob"]),
            TestCase(2, vec![("takahashi", 1000000000), ("aoki", 999999999)], vec!["aoki", "takahashi"]),
        ];

        for TestCase(n, sa, expected) in tests {
            assert_eq!(run(n, sa), expected);
        }
    }
}
