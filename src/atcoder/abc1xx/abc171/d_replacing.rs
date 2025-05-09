// https://atcoder.jp/contests/abc171/tasks/abc171_d

use std::collections::HashMap;

fn run(_n: usize, a: Vec<usize>, _q: usize, bc: Vec<(isize, isize)>) -> Vec<isize> {
    let mut hash_map: HashMap<isize, isize> = HashMap::new();

    for i in a.into_iter() {
        *hash_map.entry(i.try_into().unwrap()).or_insert(0) += 1;
    }

    let mut sum = 0;

    for (k, v) in &hash_map {
        sum += k * v;
    }

    let mut ans = Vec::new();

    for (b, c) in bc.into_iter() {
        if let Some(&count) = hash_map.get(&b) {
            // bのカウントを削除してcのカウントに加算
            *hash_map.entry(c.try_into().unwrap()).or_insert(0) += count;
            hash_map.remove(&b);

            sum += (c - b) * count;
        }

        ans.push(sum);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize, Vec<(isize, isize)>, Vec<isize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![1, 2, 3, 4], 3, vec![(1, 2), (3, 4), (2, 4)], vec![11, 12, 16]),
            TestCase(4, vec![1, 1, 1, 1], 3, vec![(1, 2), (2, 1), (3, 5)], vec![8, 4, 4]),
            TestCase(2, vec![1, 2], 3, vec![(1, 100), (2, 100), (100, 1000)], vec![102, 200, 2000]),
        ];

        for TestCase(n, a, q, bc, expected) in tests {
            assert_eq!(run(n, a, q, bc), expected);
        }
    }
}
