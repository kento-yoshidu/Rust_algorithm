// https://atcoder.jp/contests/abc360/tasks/abc360_c

pub fn run(n: usize, a: Vec<usize>, w: Vec<usize>) -> usize {
    let sum: usize = w.iter().sum();

    let mut vec = vec![(0, 0); n];

    for i in 0..n {
        vec[a[i]-1].0 += 1;

        if vec[a[i]-1].1 < w[i] {
            vec[a[i]-1].1 = w[i];
        }
    }

    sum - vec.into_iter()
        .map(|(_, v)| v)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![2, 2, 3, 3, 5], vec![33, 40, 2, 12, 16], 35),
            TestCase(12, vec![3, 6, 7, 4, 12, 4, 8, 11, 11, 1, 8, 11], vec![3925, 9785, 9752, 3587, 4013, 1117, 3937, 7045, 6437, 6208, 3391, 6309], 17254),
        ];

        for TestCase(n, a, e, expected) in tests {
            assert_eq!(run(n, a, e), expected);
        }
    }
}
