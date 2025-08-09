// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ba

use std::collections::BTreeSet;

fn run(_n: usize, q: Vec<(usize, Option<usize>)>) -> Vec<usize> {
    let mut ans = Vec::new();
    let mut btree_set = BTreeSet::new();

    for (i, price) in q.into_iter() {
        match i {
            1 => {
                btree_set.insert(price.unwrap());
            },
            2 => {
                ans.push(*btree_set.first().unwrap());
            },
            3 => {
                btree_set.pop_first();
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, Option<usize>)>, Vec<usize>);

    #[test]
    fn tessoku_a53() {
        let tests = [
            TestCase(3, vec![(1, Some(2420)), (1, Some(1650)), (2, None)], vec![1650]),
        ];

        for TestCase(n, q, expected) in tests {
            assert_eq!(run(n, q), expected);
        }
    }
}
