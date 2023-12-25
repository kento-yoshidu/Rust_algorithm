// https://atcoder.jp/contests/abc304/tasks/abc304_a

pub fn run<'a>(n: usize, sa: Vec<(&'a str, usize)>) -> Vec<&'a str> {
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

    #[test]
    fn test() {
        assert_eq!(vec!["carol", "dave", "ellen", "alice", "bob"], run(5, vec![("alice", 31), ("bob", 41), ("carol", 5), ("dave", 92), ("ellen", 65)]));
        assert_eq!(vec!["aoki", "takahashi"], run(2, vec![("takahashi", 1000000000), ("aoki", 999999999)]));
    }
}
