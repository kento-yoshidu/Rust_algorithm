// https://atcoder.jp/contests/abc194/tasks/abc194_c

use std::collections::HashMap;

fn run(_n: usize, a: Vec<isize>) -> isize {
    let mut hash_map = HashMap::new();

    for i in a {
        *hash_map.entry(i).or_insert(0) += 1;
    }

    let vec: Vec<(isize, usize)> = hash_map.into_iter().collect();

    let mut ans = 0;

    for i in 0..vec.len() {
        for j in i+1..vec.len() {
            ans += (vec[i].0 - vec[j].0).pow(2) * vec[i].1 as isize * vec[j].1 as isize;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn abc194_c() {
        let tests = [
            TestCase(3, vec![2, 8, 4], 56),
            TestCase(5, vec![-5, 8, 9, -4, -3], 950),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
