// https://atcoder.jp/contests/abc293/tasks/abc293_b

use itertools::Itertools;

fn run(n: usize, a: Vec<usize>) -> (usize, Vec<usize>) {
    let mut vec = vec![false; n];

    for i in 0..n {
        if vec[i] == false {
            vec[a[i]-1] = true;
        }
    }

    let ans: Vec<usize> = vec.iter()
        .positions(|i| *i == false)
        .map(|i| i+1)
        .collect();

    (ans.len(), ans )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, (usize, Vec<usize>));

    #[test]
    fn abc293_b() {
        let tests = [
            TestCase(5, vec![3, 1, 4, 5, 4], (2, vec![2, 4])),
            TestCase(20, vec![9, 7, 19, 7, 10, 4, 13, 9, 4, 8, 10, 15, 16, 3, 18, 19, 12, 13, 2, 12], (10, vec![1, 2, 5, 6, 8, 11, 14, 17, 18, 20])),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
