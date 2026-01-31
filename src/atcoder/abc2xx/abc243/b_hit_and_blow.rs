// https://atcoder.jp/contests/abc243/tasks/abc243_b

fn run(_n: usize, a: Vec<usize>, b: Vec<usize>) -> (usize, usize) {
    let mut count_a = 0;
    let mut count_b = 0;

    for (i, aa) in a.iter().enumerate() {
        for (j, bb) in b.iter().enumerate() {
            if i == j && aa == bb {
                count_a += 1;
            } else if aa == bb {
                count_b += 1;
            }
        }
    }

    (count_a, count_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, (usize, usize));

    #[test]
    fn abc243_b() {
        let tests = [
            TestCase(4, vec![1, 3, 5, 2], vec![2, 3, 1, 4], (1, 2)),
            TestCase(3, vec![1, 2, 3], vec![4, 5, 6], (0, 0)),
            TestCase(7, vec![4, 8, 1, 7, 9, 5, 6], vec![3, 5, 1, 7, 8, 2, 6], (3, 2)),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
