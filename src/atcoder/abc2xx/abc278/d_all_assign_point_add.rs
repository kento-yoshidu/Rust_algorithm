// https://atcoder.jp/contests/abc278/tasks/abc278_d

use std::collections::HashMap;

fn run(n: usize, a: Vec<usize>, _q: usize, q_vec: Vec<(usize, Option<usize>, Option<usize>)>) -> Vec<usize> {
    let mut hash_map = HashMap::new();

    for i in 1..=n {
        hash_map.insert(i, a[i-1]);
    }

    let mut base = 0;

    let mut ans: Vec<usize> = Vec::new();

    for (a, b, c) in q_vec.iter() {
        match a {
            1 => {
                base = b.unwrap();
                hash_map.clear()
            },
            2 => {
                hash_map.entry(b.unwrap())
                    .and_modify(|n| {
                        *n += c.unwrap();
                    })
                    .or_insert(c.unwrap());
            },
            3 => {
                ans.push(base + hash_map.get(&(b.unwrap())).unwrap_or(&0));
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize, Vec<(usize, Option<usize>, Option<usize>)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![3, 1, 4, 1, 5], 6, vec![(3, Some(2), None), (2, Some(3), Some(4)), (3, Some(3), None), (1, Some(1), None), (2, Some(3), Some(4)), (3, Some(3), None)], vec![1, 8, 5]),
            TestCase(1, vec![1000000000], 8, vec![(2, Some(1),Some(1000000000)), (2, Some(1), Some(1000000000)), (2, Some(1), Some(1000000000)), (2, Some(1), Some(1000000000)), (2, Some(1), Some(1000000000)), (2, Some(1), Some(1000000000)), (2, Some(1), Some(1000000000)), (3, Some(1), None)], vec![8000000000]),
            TestCase(10, vec![1, 8, 4, 15, 7, 5, 7, 5, 8, 0], 20, vec![(2, Some(7), Some(0)), (3, Some(7), None), (3, Some(8), None), (1, Some(7), None), (3, Some(3), None), (2, Some(4), Some(4)), (2, Some(4), Some(9)), (2, Some(10), Some(5)), (1, Some(10), None), (2, Some(4), Some(2)), (1, Some(10), None), (2, Some(3), Some(1)), (2, Some(8), Some(11)), (2, Some(3), Some(14)), (2, Some(1), Some(9)), (3, Some(8), None), (3, Some(8), None), (3, Some(1), None), (2, Some(6), Some(5)), (3, Some(7), None)], vec![7, 5, 7, 21, 21, 19, 10]),
        ];

        for TestCase(n, a, q, q_vec, expected) in tests {
            assert_eq!(run(n, a, q, q_vec), expected);
        }
    }
}
