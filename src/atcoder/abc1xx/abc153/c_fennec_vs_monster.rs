// https://atcoder.jp/contests/abc153/tasks/abc153_c

fn run(_n: usize, k: usize, h: Vec<usize>) -> usize {
    let mut vec = h.clone();

    vec.sort();
    vec.reverse();

    vec.into_iter()
        .skip(k)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc153_c() {
        let tests = [
            TestCase(3, 1, vec![4, 1, 5], 5),
            TestCase(8, 9, vec![7, 9, 3, 2, 3, 8, 4, 6], 0),
            TestCase(3, 0, vec![1000000000, 1000000000, 1000000000], 3000000000),
        ];

        for TestCase(n, k, h, expected) in tests {
            assert_eq!(run(n, k, h), expected);
        }
    }
}
