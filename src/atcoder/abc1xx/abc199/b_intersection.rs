// https://atcoder.jp/contests/abc199/tasks/abc199_b

fn run(_n: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    let min = a.iter().min().unwrap();
    let max = b.iter().max().unwrap();

    (*min..=*max)
        .filter(|num| {
            a.iter()
                .zip(b.iter())
                .all(|(l, r)| {
                    l <= num && num <= r
                })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn abc199_b() {
        let tests = [
            TestCase(2, vec![3, 2], vec![7, 5], 3),
            TestCase(3, vec![1, 5, 3], vec![10, 7, 3], 0),
            TestCase(3, vec![3, 2, 5], vec![6, 9, 8], 2),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
