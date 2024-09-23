// https://atcoder.jp/contests/abc366/tasks/abc366_c

use std::collections::HashMap;

pub fn run(_q: usize, query: Vec<(usize, Option<usize>)>) -> Vec<usize> {
    let mut ans = Vec::new();

    let mut hash_map = HashMap::new();

    for v in query.into_iter() {
        match v.0 {
            1 => {
                *hash_map.entry(v.1).or_insert(0) += 1;
            },
            2 => {
                let entry = hash_map.get_mut(&v.1).unwrap();

                *entry -= 1;

                if *entry == 0 {
                    hash_map.remove(&v.1);
                }
            },
            3 => {
                ans.push(hash_map.len())
            },
            _ => unreachable!()
        }
    }

    println!("{:?}", ans);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, Option<usize>)>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, vec![(1, Some(3)), (1, Some(1)), (1, Some(4)), (3, None), (2, Some(1)), (3, None), (1, Some(5)), (3, None)], vec![3, 2, 3]),
            TestCase(8, vec![(1, Some(2)), (1, Some(2)), (3, None), (2, Some(2)), (1, Some(4)), (1, Some(4)), (2, Some(2)), (3, None)], vec![1, 1]),
        ];

        for TestCase(q, query, expected) in tests {
            assert_eq!(run(q, query), expected);
        }
    }
}
