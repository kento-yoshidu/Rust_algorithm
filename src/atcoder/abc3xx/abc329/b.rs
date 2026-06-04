// https://atcoder.jp/contests/abc329/tasks/abc329_b

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut vec = a.clone();

    vec.sort();
    vec.dedup();
    vec.reverse();

    vec[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc329_b() {
        let tests = [
            TestCase(5, vec![2, 1, 3, 3, 2], 2),
            TestCase(4, vec![4, 3, 2, 1], 3),
            TestCase(8, vec![22, 22, 18, 16, 22, 18, 18, 22], 18),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
