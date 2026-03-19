// https://yukicoder.me/problems/no/135

use itertools::Itertools;

fn run(_n: usize, x: Vec<isize>) -> isize {
    x.into_iter()
        .sorted()
        .dedup()
        .tuple_windows()
        .map(|(x, y)| y - x)
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn yuki_135() {
        let tests = [
            TestCase(3, vec![0, 51, 100], 49),
            TestCase(4, vec![0, 1, 1, 0], 1),
            TestCase(4, vec![1, 1, 1, 1], 0),
        ];

        for TestCase(n, x, expected) in tests {
            assert_eq!(run(n, x), expected);
        }
    }
}
