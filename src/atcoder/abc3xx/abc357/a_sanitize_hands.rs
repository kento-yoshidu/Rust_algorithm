// https://atcoder.jp/contests/abc357/tasks/abc357_a

fn run(n: usize, m: usize, h: Vec<usize>) -> usize {
    let mut rest = m;

    for (i, num) in h.into_iter().enumerate() {
        if rest < num {
            return i;
        }

        rest -= num;
    }

    return n;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 10, vec![2, 3, 2, 5, 3], 3),
            TestCase(5, 10, vec![2, 3, 2, 3, 5], 4),
            TestCase(1, 5, vec![1], 1),
        ];

        for TestCase(n, m, h, expected) in tests {
            assert_eq!(run(n, m, h), expected);
        }
    }
}
