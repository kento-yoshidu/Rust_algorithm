// https://yukicoder.me/problems/no/185

fn run(_n: usize, xy: Vec<(usize, usize)>) -> isize {
     let diffs: Vec<_> = xy.into_iter()
        .map(|(x, y)| y as isize - x as isize)
        .collect();

    diffs.first().copied()
        .filter(|&first| first > 0)
        .filter(|&first| diffs.iter().all(|&d| d == first))
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, isize);

    #[test]
    fn yuki_185() {
        let tests = [
            TestCase(2, vec![(3, 5), (7, 9)], 2),
            TestCase(2, vec![(3, 5), (7, 10)], -1),
            TestCase(1, vec![(8, 2)], -1),
        ];

        for TestCase(n, xy, expected) in tests {
            assert_eq!(run(n, xy), expected);
        }
    }
}
