// https://atcoder.jp/contests/abc368/tasks/abc368_b

use std::cmp::min;

fn run(_n: usize, a: &Vec<usize>) -> usize {
    let mut vec = a.clone();

    let mut ans = 0;

    loop {
        vec.sort_by(|a, b| b.cmp(a));

        if vec[0] == 0 || vec[1] == 0 {
            break;
        }

        vec[0] -= 1;
        vec[1] -= 1;

        ans += 1;
    }

    ans
}

fn run2(_n: usize, a: &Vec<usize>) -> usize {
    let max = a.iter().max().unwrap();
    let sum: usize = a.iter().sum();

    min(sum/2, sum-max)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![1, 2, 3, 3], 4),
            TestCase(3, vec![1, 1, 100], 2),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, &a), expected);
            assert_eq!(run2(n, &a), expected);
        }
    }
}
