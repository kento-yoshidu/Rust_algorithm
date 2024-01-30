// https://atcoder.jp/contests/abc019/tasks/abc019_1

use itertools::Itertools;

pub fn run(a: usize, b: usize, c: usize) -> usize {
    *vec![a, b, c]
        .iter()
        .sorted()
        .skip(1)
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(2, 3, 4, 3)]
    #[case(5, 100, 5, 5)]
    #[case(3, 3, 3, 3)]
    #[case(3, 3, 4, 3)]
    fn test(#[case] a: usize, #[case] b: usize, #[case] c: usize, #[case] expected: usize) {
        assert_eq!(expected, run(a, b, c));
    }
}
