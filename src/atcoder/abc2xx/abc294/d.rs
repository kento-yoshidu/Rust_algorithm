// https://atcoder.jp/contests/abc294/tasks/abc294_d

use std::collections::BTreeSet;

fn run(_n: usize, _q: usize, event: Vec<(usize, Option<usize>)>) -> Vec<usize> {
    let mut ans = Vec::new();

    let mut bt_set = BTreeSet::new();

    let mut current = 0;

    for (e, x) in event {
        match e {
            1 => {
                bt_set.insert(current + 1);
                current += 1;
            },
            2 => {
                bt_set.remove(&x.unwrap());
            },
            3 => {
                let min = bt_set.iter().min().unwrap();
                ans.push(*min);
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, Option<usize>)>, Vec<usize>);

    #[test]
    fn abc294_d() {
        let tests = [
            TestCase(4, 10, vec![(1, None), (1, None), (3, None), (2, Some(1)), (1, None), (2, Some(3)), (3, None), (1, None), (2, Some(2)), (3, None)], vec![1, 2, 4]),
        ];

        for TestCase(n, q, event, expected) in tests {
            assert_eq!(run(n, q, event), expected);
        }
    }
}
