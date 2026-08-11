
// https://atcoder.jp/contests/abc333/tasks/abc333_c?lang=ja

use itertools::Itertools;

fn run(n: usize) -> usize {
    let vec: Vec<usize> =
        (1..=12)
            .map(|i| {
                ("1".to_string().repeat(i)).parse::<usize>().unwrap()
            })
            .collect();

    vec.into_iter()
        .combinations_with_replacement(3)
        .map(|arr| arr.into_iter().sum::<usize>())
        .sorted()
        .nth(n-1)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn abc333_c() {
        let tests = [
            TestCase(5, 113),
            TestCase(19, 2333),
            TestCase(333, 112222222233),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
