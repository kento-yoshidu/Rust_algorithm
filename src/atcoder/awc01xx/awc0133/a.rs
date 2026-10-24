// https://atcoder.jp/contests/awc0133/tasks/awc0133_a

fn run(n: usize, _m: usize, k: usize, t: Vec<usize>, d: Vec<usize>) -> usize {
    let mut arr = vec![false; n + 1];

    for d in d {
        arr[d] = true;
    }

    t.into_iter()
        .enumerate()
        .filter_map(|(i, t)| {
            (!arr[i+1]).then_some(t / k)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn awc0133_a() {
        let tests = [
            TestCase(5, 2, 3, vec![10, 7, 8, 5, 6], vec![2, 4], 7),
            TestCase(4, 1, 5, vec![3, 12, 7, 20], vec![3], 6),
            TestCase(10, 3, 100, vec![250, 99, 450, 1000, 50, 333, 678, 812, 999, 10], vec![2, 5, 9], 33),
            TestCase(15, 5, 7, vec![100, 49, 35, 77, 64, 21, 88, 53, 42, 99, 15, 70, 28, 56, 91], vec![1, 5, 8, 12, 15], 72),
            TestCase(1, 0, 1000000000, vec![999999999], vec![], 0),
        ];

        for TestCase(n, m, k, t, d, expected) in tests {
            assert_eq!(run(n, m, k, t, d), expected);
        }
    }
}
