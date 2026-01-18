// https://atcoder.jp/contests/abc441/tasks/abc441_c

use itertools::Itertools;

fn run(n: usize, k: usize, x: usize, a: Vec<usize>) -> isize {
    let a: Vec<usize> = a.into_iter().sorted().collect();

    let mut index = a.len();
    index -= n - k;

    let mut ans = (n - k) as isize;

    if a[..index].iter().sum::<usize>() < x {
        return -1;
    }

    let mut sum = 0;

    while sum < x {
        index -= 1;
        sum += a[index];
        ans += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, isize);

    #[test]
    fn abc441_c() {
        let tests = [
            TestCase(3, 2, 5, vec![10, 6, 8], 2),
            TestCase(2, 1, 8, vec![6, 10], -1),
            TestCase(5, 3, 3000000000, vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000], 5),
        ];

        for TestCase(n, k, x, a, expected) in tests {
            assert_eq!(run(n, k, x, a), expected);
        }
    }
}
