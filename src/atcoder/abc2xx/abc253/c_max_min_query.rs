// https://atcoder.jp/contests/abc253/tasks/abc253_c

use std::collections::BTreeMap;
use std::cmp::min;

fn run(_q: usize, query: Vec<(usize, Option<usize>, Option<usize>)>) -> Vec<usize> {
    let mut bt = BTreeMap::new();

    let mut ans = Vec::new();

    for tup in query.iter() {
        match tup {
            (1, Some(b), None) => {
                *bt.entry(b).or_insert(0) += 1;
            },
            (2, Some(b), Some(c)) => {
                if let Some(value) = bt.get_mut(b) {
                    let subtract_amount = min(*value, *c);
                    *value -= subtract_amount;

                    if *value == 0 {
                        bt.remove(b);
                    }
                }
            },
            (3, None, None) => {
                ans.push(*bt.iter().next_back().unwrap().0 - *bt.iter().next().unwrap().0);
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, Option<usize>, Option<usize>)>, Vec<usize>);

    #[test]
    fn abc253_c() {
        let tests = [
            TestCase(8, vec![(1, Some(3), None), (1, Some(2), None), (3, None, None), (1, Some(2), None), (1, Some(7), None), (3, None, None), (2, Some(2), Some(3)), (3, None, None)], vec![1, 5, 4]),
            TestCase(4, vec![(1, Some(10000), None), (1, Some(1000), None), (2, Some(100), Some(3)), (1, Some(10), None)], vec![]),
        ];

        for TestCase(q, query, expected) in tests {
            assert_eq!(run(q, query), expected);
        }
    }
}
