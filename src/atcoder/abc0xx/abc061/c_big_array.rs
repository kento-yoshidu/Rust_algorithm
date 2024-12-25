// https://atcoder.jp/contests/abc061/tasks/abc061_c

fn run(_n: usize, k: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut vec = ab.clone();

    vec.sort_by(|a, b| a.0.cmp(&b.0));

    let mut rest = k;

    for i in vec {
        if rest <= i.1 {
            return i.0
        } else {
            rest -= i.1
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 4, vec![(1, 1), (2, 2), (3, 3)], 3),
            TestCase(10, 500000, vec![(1, 100000), (1, 100000), (1, 100000), (1, 100000), (1, 100000), (100000, 100000), (100000, 100000), (100000, 100000), (100000, 100000), (100000, 100000)], 1),
        ];

        for TestCase(n, k, ab, expected) in tests {
            assert_eq!(run(n, k, ab), expected);
        }
    }
}
