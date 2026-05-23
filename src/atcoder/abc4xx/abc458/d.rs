// https://atcoder.jp/contests/abc458/tasks/abc458_d

use std::{cmp::Reverse, collections::BinaryHeap};

fn add(v: usize, l: &mut BinaryHeap<usize>, r: &mut BinaryHeap<Reverse<usize>>) {
    if let Some(&Reverse(head)) = r.peek() {
        if v < head {
            l.push(v);
        } else {
            if l.len() == r.len() {
                r.push(Reverse(v));
            } else {
                l.push(head);
                r.pop();
                r.push(Reverse(v));
            }
        }

        if l.len() > r.len() {
            let x = l.pop().unwrap();
            r.push(Reverse(x));
        }
    }
}

fn run(x: usize, _q: usize, ab: Vec<(usize, usize)>) -> Vec<usize> {
    let mut l = BinaryHeap::new();
    let mut r = BinaryHeap::new();

    r.push(Reverse(x));

    ab.into_iter()
        .map(|(a, b)| {
            add(a, &mut l, &mut r);
            add(b, &mut l, &mut r);

            r.peek().unwrap().0
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, Vec<usize>);

    #[test]
    fn abc458_d() {
        let tests = [
            TestCase(5, 3, vec![(2, 3), (1, 2), (8, 9)], vec![3, 2, 3]),
            TestCase(1, 4, vec![(2, 3), (4, 5), (6, 7), (8, 9)], vec![2, 3, 4, 5]),
            TestCase(278117031, 7, vec![(167642909, 517897721), (148434323, 567739597), (319926999, 481642530), (659199879, 252516557), (49913403, 798318034), (89701408, 892537201), (199166668, 742285869)], vec![278117031, 278117031, 319926999, 319926999, 319926999, 319926999, 319926999]),
        ];

        for TestCase(x, q, ab, expected) in tests {
            assert_eq!(run(x, q, ab), expected);
        }
    }
}
