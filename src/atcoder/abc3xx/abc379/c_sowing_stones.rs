// https://atcoder.jp/contests/abc379/tasks/abc379_c

use std::collections::BTreeMap;

pub fn run(n: usize, m: usize, x: Vec<usize>, a: Vec<usize>) -> isize {
    if n != a.iter().sum() {
        return -1;
    }

    let mut map = BTreeMap::from_iter(x.into_iter().zip(a.into_iter()));
    map.insert(n + 1, 0);

    let entries: Vec<(usize, usize)> = map.into_iter().collect();

    let mut ans = 0;
    let mut rem = 0;

    for i in 1..=m {
        let diff = entries[i].0 - entries[i-1].0;

        if rem + entries[i-1].1 < diff {
            return -1;
        }

        ans += diff * (diff-1) / 2;
        rem = rem + entries[i-1].1 - diff;
        // 余りの移動
        ans += diff * rem;
    }


    if rem != 0 {
        -1
    } else {
        ans as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, 2, vec![1, 4], vec![3, 2], 4),
            TestCase(10, 3, vec![1, 4, 8], vec![4, 2, 4], -1),
        ];

        for TestCase(n, m, x, a, expected) in tests {
            assert_eq!(run(n, m, x, a), expected);
        }
    }
}
