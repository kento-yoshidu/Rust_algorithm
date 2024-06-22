// https://atcoder.jp/contests/abc094/tasks/arc095_a

use itertools::Itertools;

pub fn run(n: usize, x: Vec<usize>) -> Vec<usize> {
    let vec: Vec<&usize> = x.iter().sorted().collect();

    let mid1 = vec[n/2 - 1];
    let mid2 = vec[n/2];

    let mut ans = Vec::new();

    for i in 0..n {
        if x[i] <= *mid1 {
            ans.push(*mid2)
        } else {
            ans.push(*mid1)
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![2, 4, 4, 3], vec![4, 3, 3, 4]),
            TestCase(2, vec![1, 2], vec![2, 1]),
            TestCase(6, vec![5, 5, 4, 4, 3, 3], vec![4, 4, 4, 4, 4, 4]),
        ];

        for TestCase(n, x, expected) in tests {
            assert_eq!(run(n, x), expected);
        }

    }
}
