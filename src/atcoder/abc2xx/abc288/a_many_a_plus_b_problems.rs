// https://atcoder.jp/contests/abc288/tasks/abc288_a

fn run(_n: usize, ab: Vec<(isize, isize)>) -> Vec<isize> {
    ab.into_iter()
        .map(|v| {
            v.0 + v.1
        })
        .collect::<Vec<isize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(isize, isize)>, Vec<isize>);

    #[test]
    fn abc288_a() {
        let tests = [
            TestCase(4, vec![(3, 5), (2, -6), (-5, 0), (314159265, 123456789)], vec![8, -4, -5, 437616054]),
        ];

        for TestCase(n, ab, expected) in tests {
            assert_eq!(run(n, ab), expected);
        }
    }
}
