// https://atcoder.jp/contests/awc0045/tasks/awc0045_a

fn run(n: usize, tp: Vec<(&str, usize)>) -> usize {
    tp.into_iter()
        .map(|(t, p)| {
            match t {
                "normal" => p,
                "half" => p / 2,
                _ => unreachable!(),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(&'static str, usize)>, usize);

    #[test]
    fn awc0045_a() {
        let tests = [
            TestCase(3, vec![("normal", 1000), ("half", 500), ("half", 300)], 1400),
            TestCase(5, vec![("normal", 2000), ("half", 1999), ("normal", 500), ("half", 750), ("normal", 1200)], 5074),
            TestCase(10, vec![("normal", 100000), ("half", 99999), ("normal", 50000), ("half", 1), ("normal", 12345), ("half", 67890), ("normal", 1), ("half", 100000), ("normal", 99999), ("half", 55555)], 424066),
            TestCase(3, vec![("normal", 1000), ("half", 500), ("half", 300)], 1400),
        ];

        for TestCase(n, tp, expected) in tests {
            assert_eq!(run(n, tp), expected);
        }
    }
}
