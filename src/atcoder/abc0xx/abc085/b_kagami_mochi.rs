// https://atcoder.jp/contests/abc085/tasks/abc085_b

use itertools::Itertools;

fn run(_n: usize, d: &Vec<usize>) -> usize {
    let mut vec = d.clone();

    vec.sort();
    vec.dedup();

    vec.len()
}

fn run2(_n: usize, d: &Vec<usize>) -> usize {
    d.into_iter()
        .sorted()
        .dedup()
        .collect::<Vec<&usize>>()
        .len()
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![10, 8, 8, 6], 3),
            TestCase(3, vec![15, 15, 15], 1),
            TestCase(7, vec![50, 30, 50, 100, 50, 80, 30], 4),
        ];

        for TestCase(n, d, expected) in tests {
            assert_eq!(run(n, &d), expected);
            assert_eq!(run2(n, &d), expected);
        }
    }
}
